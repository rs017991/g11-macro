//! Manages the user configuration at rest, with file(s) stored
//! in the `g11-macro-daemon` subdirectory of `$XDG_CONFIG` (usually `~/.config`)

use std::{
    fs::File,
    io::{self, Write, Read, BufRead, BufReader},
    path::{PathBuf, Path},
};
use derive_more::{Display, Error};
use enigo::{Direction, agent::Token};
use log::warn;
use serde::{Deserialize, Serialize};
use ron::error::{Position, SpannedError};

#[derive(Default, Debug, PartialEq, Deserialize, Serialize)]
pub struct Config {
    pub key_bindings: Vec<KeyBinding>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct KeyBinding {
    /// The `M` key (numbered `1 ..= 3`) whose bank must be active for this binding to apply
    pub m: u8,
    /// The `G` key (numbered `1 ..= 18`)
    pub g: u8,
    /// If set to `Press`, will run the script as soon as the key is pressed. Otherwise, it will wait for release.
    pub on: Direction,
    /// The sequence of steps to be executed
    pub script: Vec<Token>,
}

pub const XDG_PREFIX: &str = "g11-macro-daemon";
pub const XDG_CONFIG_KEY_BINDINGS: &str = "key_bindings.ron";

/// Loads the [`XDG_CONFIG_KEY_BINDINGS`] file, creating an empty stub if it does not yet exist.
pub fn ensure_and_load_config_file() -> Result<Config, LoadError> {
    let app_config_dir = xdg::BaseDirectories::with_prefix(XDG_PREFIX);
    let key_bindings_path = app_config_dir.place_config_file(XDG_CONFIG_KEY_BINDINGS).map_err(LoadError::Locating)?;

    let key_bindings =
        if key_bindings_path.try_exists().map_err(LoadError::Locating)? {
            try_load_key_bindings(&key_bindings_path, || File::open(&key_bindings_path), None, false)?
        } else { //Try to create a default file with instructions/samples
            File::create_new(&key_bindings_path)
                .and_then(|mut file| file.write_all(include_bytes!("config_stub.ron")))
                .inspect_err(|err| warn!("Failed to create stub for key bindings file: {}. Ignoring...\n\tCause: {err:#?}", key_bindings_path.display()))
                .map_or_else(|_| vec![], |_| vec![])
        };

    Ok(Config { key_bindings })
}

/// Parse a key bindings file, being tolerant of one or both of the outer list brackets being absent
/// TODO This is pretty hacky and gross. Would be more appropriate to just write a parser.
fn try_load_key_bindings<R: Read>(
    key_bindings_path: &Path,
    key_bindings_file: impl Fn() -> io::Result<R>,
    replace_missing_open_bracket: Option<Position>,
    replace_missing_close_bracket: bool,
) -> Result<Vec<KeyBinding>, LoadError> {
    let mut outer_buf = String::new(); //Not terribly clean, but ensures that we don't run afoul of lifetimes.
    let reader: Box<dyn Read> = {
        let mut reader: Box<dyn Read> =
            key_bindings_file()
                .map(Box::new)
                .map_err(|err| LoadError::Loading(key_bindings_path.into(), err))?;

        if let Some(missing_open_bracket_pos) = replace_missing_open_bracket {
            let mut lines = BufReader::new(reader).lines();
            for line in lines.by_ref().take(missing_open_bracket_pos.line - 1) {
                outer_buf.push_str(&line.map_err(|err| LoadError::Loading(key_bindings_path.into(), err))?);
                outer_buf.push('\n');
            }
            outer_buf.push_str("\n[\n");
            for line in lines {
                outer_buf.push_str(&line.map_err(|err| LoadError::Loading(key_bindings_path.into(), err))?);
                outer_buf.push('\n');
            }
            reader = Box::new(BufReader::new(outer_buf.as_bytes()));
        }

        if replace_missing_close_bracket {
            reader = Box::new(reader.chain(BufReader::new(&b"\n]\n"[..])));
        }

        reader
    };

    ron::Options::default()
        .from_reader(reader)
        .or_else(|err| match (&err.code, replace_missing_open_bracket, replace_missing_close_bracket) {
            (ron::Error::ExpectedArray, None, false) => {
                warn!("Possibly missing the open bracket in {}; will attempt a more lenient parse, but you should fix the file.", key_bindings_path.display());
                try_load_key_bindings(key_bindings_path, key_bindings_file, Some(err.position), false)
                    .map_err(|_| LoadError::unable_to_parse_or_load_config(key_bindings_path.into(), err)) //If tolerant parse fails, just return the original error
            }
            (ron::Error::ExpectedStructName(name), _, false) if name == "KeyBinding" => {
                warn!("Possibly missing the close bracket in {}; will attempt a more lenient parse, but you should fix the file.", key_bindings_path.display());
                try_load_key_bindings(key_bindings_path, key_bindings_file, replace_missing_open_bracket, true)
                    .map_err(|_| LoadError::unable_to_parse_or_load_config(key_bindings_path.into(), err)) //If tolerant parse fails, just return the original error
            }
            _ => Err(LoadError::unable_to_parse_or_load_config(key_bindings_path.into(), err))
        })
}

#[derive(Debug, Display, Error)]
pub enum LoadError {
    #[display("Unable to locate the config file! Cause: {_0}")]
    Locating(io::Error),
    #[display("Unable to load the config from {}! Cause: {_1}", _0.display())]
    Loading(PathBuf, io::Error),
    #[display("Unable to parse the config from {}! Cause: {_1}", _0.display())]
    Parsing(PathBuf, SpannedError),
}
impl LoadError {
    fn unable_to_parse_or_load_config(path: PathBuf, ron_err: SpannedError) -> Self {
        if let SpannedError { code: ron::Error::Io(io_err), .. } = ron_err {
            Self::Loading(path, io::Error::other(io_err))
        } else {
            Self::Parsing(path, ron_err)
        }
    }
}


#[cfg(test)]
mod tests {
    use test_log::test;
    use enigo::Direction::*;
    use super::*;

    #[test] fn tolerant_of_missing_open_bracket(){ parses_correctly(true, false, false); }
    #[test] fn tolerant_of_missing_close_bracket(){ parses_correctly(false, true, false); }
    #[test] fn tolerant_of_missing_brackets(){ parses_correctly(true, true, false); }
    #[test] fn tolerant_of_missing_extensions(){ parses_correctly(false, false, true); }
    #[test] fn parses_correctly_when_valid() { parses_correctly(false, false, false); }
    fn parses_correctly(missing_open_bracket: bool, missing_close_bracket: bool, missing_extensions: bool) {
        let config = Config {
            key_bindings: vec![
                KeyBinding {
                    m: 1,
                    g: 1,
                    on: Press,
                    script: vec![
                        Token::Key(enigo::Key::Control,      Press),
                        Token::Key(enigo::Key::Unicode('-'), Click),
                        Token::Key(enigo::Key::Control,      Release),
                    ],
                },
                KeyBinding {
                    m: 1,
                    g: 2,
                    on: Press,
                    script: vec![
                        Token::Key(enigo::Key::Control,      Press),
                        Token::Key(enigo::Key::Unicode('0'), Click),
                        Token::Key(enigo::Key::Control,      Release),
                    ],
                },
            ],
        };

        let input = format!(r"
            {extensions}
            {open}
                KeyBinding(
                    m: 1,
                    g: 1,
                    on: Press,
                    script: [
                        Key(Control, Press),
                        Key(Unicode('-'), Click),
                        Key(Control, Release),
                    ],
                ),
                KeyBinding(
                    m: 1,
                    g: 2,
                    on: Press,
                    script: [
                        Key(Control, Press),
                        Key(Unicode('0'), Click),
                        Key(Control, Release),
                    ],
                ),
            {close}
        ", open = if missing_open_bracket { "" } else { "[" }, close = if missing_close_bracket { "" } else { "]" },
           extensions = if missing_extensions { "" } else { "#![enable(explicit_struct_names, implicit_some)]" },
        );

        let parsed = try_load_key_bindings(Path::new("n/a"), || Ok(BufReader::new(input.as_bytes())), None, false).expect("does not fail to parse");

        assert_eq!(parsed, config.key_bindings);
    }

    #[test]
    fn prebaked_stub_is_valid() {
        let parsed = ron::from_str::<Vec<KeyBinding>>(include_str!("config_stub.ron"))
            .expect("does not fail to parse");

        assert_eq!(parsed, vec![]);
    }
}
