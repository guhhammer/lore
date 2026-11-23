# systemd-ask-password

Query the user for a system password.

## Usage
- `systemd-ask-password "prompt"` : query a system password with a specific prompt
- `systemd-ask-password --id identifier "prompt"` : specify an identifier for the password query
- `systemd-ask-password --keyname key_name "prompt"` : use a kernel keyring key name as a cache for the password
- `systemd-ask-password --timeout seconds "prompt"` : set a custom timeout for the password query
- `systemd-ask-password --no-tty "prompt"` : force the use of an agent system and never ask on current TTY
- `systemd-ask-password --no-output --keyname key_name "prompt"` : store a password in the kernel keyring without displaying it
- `systemd-ask-password -n | command` : pipe a password to a command, stripping the trailing newline
- `systemd-ask-password -h` : display help

## Categories
users, system, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-ask-password
