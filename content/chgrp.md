# chgrp

Change group ownership of files and directories.

## Usage
- `chgrp group path/to/file_or_directory` : change the owner group of a file/directory
- `chgrp -R group path/to/directory` : recursively change the owner group of a directory and its contents
- `chgrp -h group path/to/symlink` : change the owner group of a symbolic link
- `chgrp --reference path/to/reference_file path/to/file_or_directory` : change the owner group of a file/directory to match a reference file

## See also
- chown

## Categories
files, users

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/chgrp
