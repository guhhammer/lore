# runlim

Sample and limit time and memory usage of a program and its child processes using the proc filesystem on Linux.

## Usage
- `runlim command command_arguments` : print the time and memory usage of a command
- `runlim --output-file=path/to/file command command_arguments` : log statistics to a file instead of `stdout`
- `runlim --time-limit=number command command_arguments` : limit time to an upper bound (in seconds)
- `runlim --real-time-limit=number command command_arguments` : limit real-time to an upper bound (in seconds)
- `runlim --space-limit=number command command_arguments` : limit space to an upper bound (in MB)

## Categories
monitoring, files, disks

## Links
- More information: https://fmv.jku.at/runlim/
- tldr page: https://tldr.inbrowser.app/pages/linux/runlim
