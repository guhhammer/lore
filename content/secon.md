# secon

Get the SELinux security context of a file, pid, current execution context, or a context specification.

## Usage
- `secon` : get the security context of the current execution context
- `secon -p 1` : get the current security context of a process
- `secon -f path/to/file_or_directory` : get the current security context of a file, resolving all intermediate symlinks
- `secon -L path/to/symlink` : get the current security context of a symlink itself (i.e. do not resolve)
- `secon system_u:system_r:container_t:s0:c899,c900` : parse and explain a context specification

## See also
- semanage
- runcon
- chcon

## Categories
files, processes

## Links
- More information: https://manned.org/secon
- tldr page: https://tldr.inbrowser.app/pages/linux/secon
