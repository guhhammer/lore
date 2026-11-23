# pacrepairfile

Reset properties on files managed by alpm.

## Usage
- `pacrepairfile path/to/file --package` : search for the package and reset the properties of a file
- `pacrepairfile package_name --quiet --package` : reset a file quietly
- `pacrepairfile package_name --mode|gid|mtime|uid --package` : reset specific file properties (mode, owner UID, group GID, or modification time)
- `pacrepairfile --help` : display help
- `pacrepairfile --version` : display version

## Categories
files, packages

## Links
- More information: https://github.com/andrewgregory/pacutils/blob/master/doc/pacrepairfile.pod
- tldr page: https://tldr.inbrowser.app/pages/linux/pacrepairfile
