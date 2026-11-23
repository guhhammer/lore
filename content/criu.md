# criu

Freeze a running application or a process tree and restore it later.

## Usage
- `sudo criu check` : check if the current kernel has support for the necessary CRIU features
- `sudo criu dump -t process_id -D path/to/directory` : checkpoint a process with a specific ID, saving the state to a specific directory
- `sudo criu restore -D path/to/directory` : restore a process from previously saved image files
- `sudo criu dump -t process_id -D path/to/directory -R` : dump a process and allow it to continue running instead of terminating it

## Categories
files, processes, users

## Links
- More information: https://manned.org/criu
- tldr page: https://tldr.inbrowser.app/pages/linux/criu
