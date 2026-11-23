# pipesz

Set or examine pipe and FIFO buffer sizes.

## Usage
- `pipesz [options] [--set size] [--] [command [argument] ...]` : the general form, as the manual gives it
- `pipesz [options] --get` : another form of the same command
- `pipesz -g` : report the size of pipe buffers to standard output and exit
- `pipesz -s` : set the size of the pipe buffers, in bytes
- `pipesz -i` : shorthand for --fd 0
- `pipesz -o` : shorthand for --fd 1

## Links
- Documentation: https://www.kernel.org/pub/linux/utils/util-linux/
