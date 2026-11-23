# swaymsg

Send messages to a running instance of Sway using IPC. See <https://github.com/swaywm/sway/blob/master/sway/sway.5.scd> for available commands.

## Usage
- `swaymsg command` : run a Sway command
- `swaymsg -t get_workspaces` : display a list of workspaces
- `swaymsg -t get_inputs` : display a list of input devices
- `swaymsg -t get_outputs` : display a list of output devices
- `swaymsg -t get_tree` : display a layout tree of all open windows, containers, outputs, and workspaces

## Categories
network, desktop

## Links
- More information: https://github.com/swaywm/sway/blob/master/swaymsg/swaymsg.1.scd
- tldr page: https://tldr.inbrowser.app/pages/common/swaymsg
