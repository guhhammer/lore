# xxh

Bring your shell with all of your customizations through SSH sessions. Note: `xxh` does not install anything into system directories on the target machine; removing `~/.xxh` will clear all traces of xxh on the target machine.

## Usage
- `xxh "host"` : connect to a host and run the current shell
- `xxh "host" ++install` : install the current shell into the target machine without prompting
- `xxh "host" ++shell xonsh|zsh|fish|bash|osquery` : run the specified shell on the target machine
- `xxh "host" ++host-xxh-home ~/.xxh` : use a specific xxh configuration directory on the target machine
- `xxh "host" ++xxh-config ~/.config/xxh/config.xxhc` : use the specified configuration file on the host machine
- `xxh "host" ++password "password"` : specify a password to use for the SSH connection
- `xxh "host" ++install-xxh-packages package` : install an xxh package on the target machine
- `xxh "host" ++env name=value` : set an environment variable for the shell process on the target machine

## Categories
network, packages, shell

## Links
- More information: https://github.com/xxh/xxh#usage
- tldr page: https://tldr.inbrowser.app/pages/common/xxh
