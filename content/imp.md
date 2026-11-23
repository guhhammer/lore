# imp

A helper to use native systemd support under WSL (Windows Subsystem for Linux). Note: To run these from Windows rather than an already-running distribution, precede them with `wsl`.

## Usage
- `imp -i` : initialize the helper functions and keep WSL running until explicit shutdown (run once, at start)
- `imp -s` : run a shell inside a systemd user session
- `imp -c command` : run a specified command inside a systemd user session (preserves working directory)
- `imp -u` : shut down systemd and the WSL instance

## Categories
system

## Links
- More information: https://github.com/arkane-systems/bottle-imp#usage
- tldr page: https://tldr.inbrowser.app/pages/linux/imp
