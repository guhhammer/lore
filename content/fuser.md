# fuser

Display process IDs currently using files or sockets.

## Usage
- `fuser path/to/file_or_directory` : find which processes are accessing a file or directory
- `fuser -v path/to/file_or_directory` : show more fields (`USER`, `PID`, `ACCESS`, and `COMMAND`)
- `fuser port/tcp` : identify processes using a TCP socket
- `fuser -k path/to/file_or_directory` : kill all processes accessing a file or directory (sends the `SIGKILL` signal)
- `fuser -m path/to/file_or_directory` : find which processes are accessing the filesystem containing a specific file or directory
- `fuser -k port/tcp` : kill all processes with a TCP connection on a specific port

## Categories
files, processes, network

## Links
- More information: https://manned.org/fuser
- tldr page: https://tldr.inbrowser.app/pages/linux/fuser
