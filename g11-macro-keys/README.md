# g11-macro-keys

Library that models the USB/HID interface for the Logitech G11 Keyboard's macro keys.

This was primarily written for use within [g11-macro-daemon](https://crates.io/crates/g11-macro-daemon),
but you may check out the [examples](examples) to see how you could build your own weird stuff on top of this library.  

## Key Layout
The macro keys interface covers:
* 18 'G' keys
* 3 'M' keys - _each has its own LED_
* 1 'MR' key - _has its own LED_
* 1 Backlight key
```text
  (M1) (M2) (M3)  (MR)   ...   (💡)

 ┌───┐┌───┐┌───┐
 │G1 ││G2 ││G3 │
 └───┘└───┘└───┘
 ┌───┐┌───┐┌───┐
 │G4 ││G5 ││G6 │
 └───┘└───┘└───┘

 ┌───┐┌───┐┌───┐
 │G7 ││G8 ││G9 │
 └───┘└───┘└───┘
 ┌───┐┌───┐┌───┐
 │G10││G11││G12│
 └───┘└───┘└───┘

 ┌───┐┌───┐┌───┐
 │G13││G14││G15│
 └───┘└───┘└───┘
 ┌───┐┌───┐┌───┐
 │G16││G17││G18│
 └───┘└───┘└───┘
```
