# openvt

Start a program on a new virtual terminal (VT).

## Usage
- `openvt [-c vtnumber] [OPTIONS] [--] command` : the general form, as the manual gives it
- `openvt -c` : use the given VT number and not the first available
- `openvt -f` : force opening a VT without checking whether it is already in use
- `openvt -e` : directly execute the given command, without forking
- `openvt -s` : switch to the new VT when starting the command

## Categories
shell
