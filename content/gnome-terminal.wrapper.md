# gnome-terminal.wrapper

X-terminal-emulator interface for gnome-terminal.

## Usage
- `x-terminal-emulator [-T TITLE] [-e COMMAND [ARGUMENTS...]]` : the general form, as the manual gives it
- `gnome-terminal.wrapper -T TITLE` : set the title of the terminal to TITLE, as specified in Debian Policy
- `gnome-terminal.wrapper -display X11DISPLAY` : if using X11, select X11DISPLAY as the X11 display to use
- `gnome-terminal.wrapper -ls Using` : this  option  is  treated  as a command-line parsing error
- `gnome-terminal.wrapper -n ICON` : accepted for command-line compatibility with xterm(1), but ignored

## Categories
shell, desktop, network

## Links
- Documentation: https://www.debian.org/doc/debian-policy/ch-customized-programs.html
