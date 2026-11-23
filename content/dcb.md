# dcb

Show / manipulate DCB (Data Center Bridging) settings.

## Usage
- `dcb [ OPTIONS ] { app | buffer | ets | maxrate | pfc } { COMMAND | help }` : the general form, as the manual gives it
- `dcb [ -force ] -batch filename` : another form of the same command
- `dcb [ OPTIONS ] help` : another form of the same command
- `dcb -n` : switches dcb to the specified network namespace NETNS
- `dcb -V` : print the version of the dcb utility and exit
- `dcb -b` : read commands from provided file or standard input and invoke them
- `dcb -f` : don't  terminate  dcb on errors in batch mode
