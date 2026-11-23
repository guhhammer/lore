# sudoedit

This command is an alias of `sudo --edit`.

## Usage
- `sudo less /var/log/syslog` : run a command as the superuser
- `sudo -e /etc/fstab` : edit a file as the superuser with your default editor
- `sudo -u user -g group id -a` : run a command as another user and/or group
- `sudo !!` : repeat the last command prefixed with `sudo` (only in Bash, Zsh, etc.)
- `sudo -i` : launch the default shell with superuser privileges and run login-specific files (`.profile`, `.bash_profile`, etc.)
- `sudo -s` : launch the default shell with superuser privileges without changing the environment
- `sudo -i -u user` : launch the default shell as the specified user, loading the user's environment and reading login-specific files (`.profile`, `.bash_profile`, etc.)
- `sudo -ll` : list the allowed (and forbidden) commands for the invoking user in longer format

## Categories
users, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/sudoedit
