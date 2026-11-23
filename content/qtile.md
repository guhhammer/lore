# qtile

A full-featured, hackable tiling window manager written and configured in Python.

## Usage
- `qtile start` : start the window manager, if it is not running already (should ideally be run from `.xsession` or similar)
- `qtile check` : check the configuration file for any compilation errors (default location is `~/.config/qtile/config.py`)
- `qtile top --force` : show current resource usage information
- `qtile run-cmd --group test_group --float xterm` : open the program `xterm` as a floating window on the group named `test-group`
- `qtile cmd-obj --object cmd --function restart` : restart the window manager

## Categories
desktop, users

## Links
- More information: https://docs.qtile.org/en/latest/manual/commands/shell/index.html
- tldr page: https://tldr.inbrowser.app/pages/linux/qtile
