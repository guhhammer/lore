# until

Simple shell loop that repeats until it receives zero as return value.

## Usage
- `until command; do :; done` : execute a command until it succeeds
- `until ssh username@host; do sleep 2; done` : try to connect to an SSH host until gracefully disconnected
- `until systemctl is-active -q nginx; do echo "Waiting..."; sleep 1; done; echo "Launched!"` : wait for a systemd service to be active

## Categories
network, shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-until
- tldr page: https://tldr.inbrowser.app/pages/common/until
