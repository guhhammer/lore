# coresched

Manage core scheduling cookies for tasks.

## Usage
- `coresched [get] [-s pid]` : the general form, as the manual gives it
- `coresched new [-t type] -d pid` : another form of the same command
- `coresched new [-t type] -- command [argument...]` : another form of the same command
- `coresched copy [-s pid] [-t type] -d pid` : another form of the same command
- `coresched copy [-s pid] [-t type] -- command [argument...]` : another form of the same command
- `coresched -s` : which PID to get the cookie from
- `coresched -d` : which PID to modify the cookie of

## Categories
processes

## Links
- Documentation: https://www.kernel.org/pub/linux/utils/util-linux/
