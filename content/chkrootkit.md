# chkrootkit

Scan system for rootkits.

## Usage
- `chkrootkit -q` : enable quiet mode and suppress normal test results
- `chkrootkit -x` : enable expert mode and produce additional outputs
- `chkrootkit -d` : enable debug mode to show all output
- `chkrootkit -e "path/to/file"` : specify excluded files for some tests
- `chkrootkit -r path/to/directory` : specify a directory as the root for testing (e.g. mounted `ext` drives)
- `chkrootkit -n` : ignore nfs-mounted directories
- `chkrootkit -T filesystemtype` : invoke Tests and ignore specific filesystem types
- `chkrootkit -l` : generate list of available tests

## Categories
files, development

## Links
- More information: https://manned.org/chkrootkit
- tldr page: https://tldr.inbrowser.app/pages/common/chkrootkit
- Wikipedia: https://en.wikipedia.org/wiki/Chkrootkit
