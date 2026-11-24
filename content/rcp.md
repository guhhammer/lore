# rcp

Copy files between local and remote systems. It mimics the behavior of the `cp` command but operates across different machines.

## Usage
- `rcp path/to/local_file username@remote_host:/path/to/destination/` : copy a file to a remote host
- `rcp -r path/to/local_directory username@remote_host:/path/to/destination/` : copy a directory recursively
- `rcp -p path/to/local_file username@remote_host:/path/to/destination/` : preserve the file attributes
- `rcp -f path/to/local_file username@remote_host:/path/to/destination/` : force copy without a confirmation

## Categories
files, network

## Links
- More information: https://www.gnu.org/software/inetutils/manual/inetutils.html#rcp-invocation
- tldr page: https://tldr.inbrowser.app/pages/linux/rcp
- Wikipedia: https://en.wikipedia.org/wiki/Berkeley_r-commands
