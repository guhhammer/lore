# alacritty

Cross-platform, GPU-accelerated terminal emulator.

## Usage
- `alacritty` : start a new Alacritty process and create a window
- `alacritty --daemon` : start the Alacritty daemon (without creating a window)
- `alacritty msg create-window` : create a new window using the already running Alacritty process
- `alacritty --working-directory path/to/directory` : start the shell in a specific directory (also works with `alacritty msg create-window`)
- `alacritty -e command` : execute a command in a new Alacritty window (also works with `alacritty msg create-window`)
- `alacritty --config-file path/to/config.toml` : use an alternative configuration file (defaults to `$XDG_CONFIG_HOME/alacritty/alacritty.toml`)

## Categories
desktop, files, shell

## Links
- More information: https://manned.org/alacritty
- tldr page: https://tldr.inbrowser.app/pages/common/alacritty
- Wikipedia: https://en.wikipedia.org/wiki/Alacritty
