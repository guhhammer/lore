# swaynag

Display a message in a bar on top of a screen.

## Usage
- `swaynag -m "error_message"` : display an error
- `swaynag -t warning -m "warning_message"` : display a warning
- `swaynag -t custom_type -m "message"` : display a message with custom settings defined in the config
- `swaynag -f "monospace bold 9" -m "error_message"` : use the specified font
- `TERMINAL=terminal_executable swaynag -b "button_text" command -m "error_message"` : create a button and run a command in the specified terminal on click
- `swaynag -B "button_text" command -m "error_message"` : create a button and run a command on click
- `swaynag -e bottom -m "error_message"` : display the bar on the bottom of the screen
- `swaynag -o DP-1 -m "error_message"` : open `swaynag` on the specified monitor

## Categories
desktop

## Links
- More information: https://github.com/swaywm/sway/blob/master/swaynag/swaynag.1.scd
- tldr page: https://tldr.inbrowser.app/pages/common/swaynag
