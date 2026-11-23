# runuser

Run commands as a user and group without asking for password.

## Usage
- `sudo runuser user -c 'command'` : run command as a different user
- `sudo runuser user -g group -c 'command'` : run command as a different user and group
- `sudo runuser user -l` : start a login shell as a specific user
- `sudo runuser user -s /bin/sh` : specify a shell for running instead of the default shell (also works for login)
- `sudo runuser user -p -c 'command'` : preserve the entire environment of root (only if `--login` is not specified)

## Categories
users, shell

## Links
- More information: https://manned.org/runuser
- tldr page: https://tldr.inbrowser.app/pages/linux/runuser
