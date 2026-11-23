# xkbcomp

Compiles keyboard layout descriptions, and can dump or load the current X keymap.

## Usage
- `xkbcomp :0 layout.xkb` : save the current keymap to a file
- `xkbcomp layout.xkb :0` : load a keymap from a file

## Notes
A bad keymap can make the keyboard unusable until you restart the X session.

## Categories
hardware, monitoring
