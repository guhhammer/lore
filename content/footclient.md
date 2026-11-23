# footclient

Start a new terminal window handled by the `foot` server. Note: `foot --server` must be started before using the client.

## Usage
- `footclient` : start a new terminal window, wait until the window is closed, and return its exit code
- `footclient -N` : start a new terminal window and exit immediately
- `footclient command` : execute a command in a new terminal window
- `footclient -F` : start a terminal in fullscreen mode
- `footclient -w widthxheight` : start a terminal with the specified window dimensions, in pixels
- `footclient -a app_id` : start a terminal with the specified Wayland app id (default: `footclient`)
- `footclient -o section.key=value` : override a configuration option

## Categories
desktop, shell

## Links
- More information: https://manned.org/footclient
- tldr page: https://tldr.inbrowser.app/pages/common/footclient
