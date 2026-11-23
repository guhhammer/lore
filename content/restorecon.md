# restorecon

Restore SELinux security context on files/directories according to persistent rules.

## Usage
- `ls -dlZ path/to/file_or_directory` : view the current security context of a file or directory
- `restorecon path/to/file_or_directory` : restore the security context of a file or directory
- `restorecon -R -v path/to/directory` : restore the security context of a directory recursively, and show all changed labels
- `restorecon -R -T 0 -p path/to/directory` : restore the security context of a directory recursively, using all available threads, and show progress
- `restorecon -R -n -v path/to/directory` : preview the label changes that would happen without applying them

## See also
- semanage fcontext

## Categories
files

## Links
- More information: https://manned.org/restorecon
- tldr page: https://tldr.inbrowser.app/pages/linux/restorecon
