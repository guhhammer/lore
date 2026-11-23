# sulogin

Log in as root during single-user mode.

## Usage
- `sudo sulogin` : start `sulogin` on the default console
- `sudo sulogin /dev/ttyX` : start `sulogin` on a specific TTY device
- `sudo sulogin -t timeout` : set a maximum timeout(in seconds) for entering the root password before continuing normal boot
- `sudo sulogin -p` : start root's shell as a login shell
- `sudo sulogin -e` : force a root shell without asking for a password when default methods of obtaining the password fail

## Categories
users, shell, monitoring

## Links
- More information: https://manned.org/sulogin
- tldr page: https://tldr.inbrowser.app/pages/linux/sulogin
