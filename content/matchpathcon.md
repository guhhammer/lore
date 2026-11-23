# matchpathcon

Look up the persistent SELinux security context setting of a path.

## Usage
- `matchpathcon /path/to/file` : look up the persistent security context setting of an absolute path
- `matchpathcon -m file|dir|pipe|chr_file|blk_file|lnk_file|sock_file /path/to/file` : restrict lookup to settings on a specific file type
- `matchpathcon -V /path/to/file` : verify that the persistent and current security context of a path agree

## See also
- semanage fcontext
- secon
- chcon
- restorecon

## Categories
files

## Links
- More information: https://manned.org/matchpathcon.8
- tldr page: https://tldr.inbrowser.app/pages/linux/matchpathcon
