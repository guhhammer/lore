# wezterm

Wez's Terminal Emulator - a powerful cross-platform terminal emulator and multiplexer. Some subcommands such as `cli` have their own usage documentation.

## Usage
- `wezterm` : start a new Wezterm process and create a window
- `wezterm ssh user@host:port` : establish an `ssh` session in a new Wezterm window
- `wezterm connect domain_name` : connect to the multiplexer (`wezterm-mux-server`)
- `wezterm imgcat path/to/image` : output an image to the terminal
- `wezterm record` : record a terminal session as an asciicast (by default recordings are saved to `/tmp`)
- `wezterm replay path/to/cast_file` : replay an asciicast terminal session
- `wezterm --config-file path/to/config_file` : specify the configuration file to use (overrides the normal configuration file resolution)
- `wezterm help` : display help

## Categories
shell, files, network

## Links
- More information: https://wezterm.org/cli/general
- tldr page: https://tldr.inbrowser.app/pages/common/wezterm
