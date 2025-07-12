//! Manages the user configuration at rest, with file(s) stored
//! in the `g11-macro-daemon` subdirectory of `$XDG_CONFIG` (usually `~/.config`)

use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
};
use derive_more::{Display, Error};
use enigo::{Direction, agent::Token};
use log::warn;
use serde::{Deserialize, Serialize};
use ron::error::SpannedError;

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

pub fn ensure_and_load_config_file() -> Result<Config, LoadError> {
    let app_config_dir = xdg::BaseDirectories::with_prefix("g11-macro-daemon");
    let key_bindings_path = app_config_dir.place_config_file("key_bindings.ron").map_err(LoadError::Locating)?;

    let key_bindings = {
        if key_bindings_path.try_exists().map_err(LoadError::Locating)? {
            let key_bindings_file =
                File::open(&key_bindings_path)
                    .map_err(|err| LoadError::Loading(key_bindings_path.clone(), err))?;
            ron::Options::default()
                .from_reader(key_bindings_file)
                .map_err(|err| LoadError::unable_to_parse_or_load_config(key_bindings_path.clone(), err))?
        } else { //Try to create a default file with instructions/samples
            File::create_new(&key_bindings_path)
                .and_then(|mut file| file.write_all(include_bytes!("config_stub.ron")))
                .inspect_err(|err| warn!("Failed to create stub for key bindings file: {}. Ignoring...\n\tCause: {err:#?}", key_bindings_path.display()))
                .map_or_else(|_| vec![], |_| vec![])
        }
    };

    Ok(Config { key_bindings })
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
    use enigo::Direction::*;
    use super::*;

    #[test]
    fn parses_correctly() {
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

        let parsed = ron::from_str::<Vec<KeyBinding>>(r#"
            #![enable(explicit_struct_names, implicit_some)]
            [
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
            ]
        "#).expect("does not fail to parse");

        assert_eq!(parsed, config.key_bindings);
    }

    #[test]
    fn prebaked_stub_is_valid() {
        let parsed = ron::from_str::<Vec<KeyBinding>>(include_str!("config_stub.ron"))
            .expect("does not fail to parse");

        assert_eq!(parsed, vec![]);
    }
}
