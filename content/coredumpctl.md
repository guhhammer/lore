# coredumpctl

Retrieve and process saved core dumps and metadata.

## Usage
- `coredumpctl` : list all captured core dumps
- `coredumpctl list program_name` : list captured core dumps for a program
- `coredumpctl list COREDUMP_SIGNAL=1..64` : filter core dumps based on a specific signal
- `coredumpctl info process_id` : show information about the core dumps matching a program with PID
- `coredumpctl debug` : invoke debugger using the last core dump
- `coredumpctl debug program_name` : invoke debugger using the last core dump of a program
- `coredumpctl dump program_name -o path/to/file` : extract the last core dump of a program to a file
- `coredumpctl debug -A "-iex 'set debuginfod enabled on' -iex 'set pagination off' -ex bt"` : skip debuginfod and pagination prompts and then print the backtrace when using `gdb`

## Categories
development, processes, files

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/coredumpctl.html
- tldr page: https://tldr.inbrowser.app/pages/linux/coredumpctl
