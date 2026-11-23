# dtach

Run a program in a session you can detach from and reattach to.

## Usage
- `dtach -c path/to/socket bash` : start a new session and attach to it
- `dtach -n path/to/socket bash` : start a new session without attaching to it
- `dtach -a path/to/socket` : attach to an existing session
- `dtach -A path/to/socket bash` : attach to a session, creating it if it does not exist
- `<Ctrl \>` : detach from the current session
- `dtach -a path/to/socket -r winch` : attach using the `winch` redraw method (useful for full-screen programs)

## See also
- screen
- tmux
- abduco

## Categories
network, files

## Links
- More information: https://manned.org/dtach
- tldr page: https://tldr.inbrowser.app/pages/common/dtach
