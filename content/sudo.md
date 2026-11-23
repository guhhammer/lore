# sudo

Runs one command as another user, root by default: what you need to
install software, change system files or manage services. It asks for
your own password, not root's.

## Usage
- `sudo apt update` : run one command as root
- `sudo -i` : a root shell, until you type `exit`
- `sudo -u postgres psql` : run a command as another user
- `sudo !!` : the previous command again, as root (bash)
- `sudo -l` : what you are allowed to run

## Notes
Everything run under sudo can change the whole system. Only users listed
in /etc/sudoers (often the `sudo` or `wheel` group) may use it; edit that
file only with `sudo visudo`.

## See also
- pkexec
- run0
- doas

## Categories
users, system

## Links
- sudo's site: https://www.sudo.ws/docs/man/sudo.man/
