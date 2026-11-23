# agetty

Alternative `getty`: Open a `tty` port, prompt for a login name, and invoke the `/bin/login` command. It is normally invoked by `init`. Note: The baud rate is the speed of data transfer between a terminal and a device over a serial connection.

## Usage
- `agetty tty 115200` : connect `stdin` to a port (relative to `/dev`) and optionally specify a baud rate (defaults to 9600)
- `agetty -t timeout_in_seconds -` : assume `stdin` is already connected to a `tty` and set a timeout for the login
- `agetty -8 - term_var` : assume the `tty` is 8-bit, overriding the `$TERM` environment variable set by `init`
- `agetty -n -l login_program tty` : skip the login (no login) and invoke, as root, another login program instead of `/bin/login`
- `agetty -i -` : do not display the pre-login (issue) file (`/etc/issue` by default) before writing the login prompt
- `agetty -r /path/to/root_directory -H fake_host -` : change the root directory and write a specific fake host into the `utmp` file

## Categories
shell, users, network

## Links
- More information: https://manned.org/agetty
- tldr page: https://tldr.inbrowser.app/pages/linux/agetty
