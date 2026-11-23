# frpc

Connect to a `frps` server to start proxying connections on the current host. Part of `frp`.

## Usage
- `frpc` : start the service, using the default configuration file (assumed to be `frps.ini` in the current directory)
- `frpc -c path/to/frpc.toml` : start the service, using the newer TOML configuration file
- `frpc -c path/to/file` : start the service, using a specific configuration file
- `frpc verify -c path/to/file` : check if the configuration file is valid
- `frpc completion bash|fish|powershell|zsh` : print autocompletion setup script for Bash, fish, PowerShell, or Zsh
- `frpc -v` : display version

## Categories
files, network, system

## Links
- More information: https://github.com/fatedier/frp
- tldr page: https://tldr.inbrowser.app/pages/common/frpc
