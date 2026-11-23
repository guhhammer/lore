# screen

Hold a session open on a remote server. Manage multiple windows with a single SSH connection.

## Usage
- `screen` : start a new screen session
- `screen -S session_name` : start a new named screen session
- `screen -dmLS session_name command` : start a new daemon and log the output to `screenlog.x`
- `screen -ls` : show open screen sessions
- `screen -r session_name` : reattach to an open screen
- `<Ctrl a><d>` : detach from inside a screen
- `<Ctrl a><k>` : kill the current screen session
- `screen -X -S session_name quit` : kill a detached screen

## See also
- tmux
- zellij
- herdr

## Categories
shell

## Links
- More information: https://manned.org/screen
- tldr page: https://tldr.inbrowser.app/pages/common/screen
