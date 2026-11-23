# frps

Quickly set up a reverse proxy service. Part of `frp`.

## Usage
- `frps` : start the service, using the default configuration file (assumed to be `frps.ini` in the current directory)
- `frps -c path/to/frps.toml` : start the service, using the newer TOML configuration file
- `frps -c path/to/file` : start the service, using a specified configuration file
- `frps verify -c path/to/file` : check if the configuration file is valid
- `frps completion bash|fish|powershell|zsh` : print autocompletion setup script for Bash, fish, PowerShell, or Zsh
- `frps -v` : display version

## Categories
files, system, network

## Links
- More information: https://github.com/fatedier/frp
- tldr page: https://tldr.inbrowser.app/pages/common/frps
