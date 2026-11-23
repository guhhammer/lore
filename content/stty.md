# stty

Set or get options for a terminal device interface.

## Usage
- `stty size` : display current terminal size
- `stty -a` : display all settings for the current terminal
- `stty rows|cols count` : set the number of rows or columns
- `stty -F path/to/device_file speed` : get the actual transfer speed of a device
- `stty sane` : reset all modes to reasonable values for the current terminal
- `stty raw|cooked` : switch between raw and normal mode
- `stty -echo|echo` : turn character echoing off or on
- `stty --help` : display help

## See also
- tput

## Categories
shell, network, hardware

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/stty-invocation.html
- tldr page: https://tldr.inbrowser.app/pages/common/stty
