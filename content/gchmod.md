# gchmod

This command is an alias of GNU `chmod`.

## Usage
- `chmod u+x path/to/file` : give the user who owns a file the right to execute it
- `chmod u+rw path/to/file_or_directory` : give the user rights to read and write to a file/directory
- `chmod g-x,o-x path/to/file` : remove executable rights from the group and others
- `chmod a+rx path/to/file` : give all users rights to read and execute
- `chmod o=g path/to/file` : give others (not in the file owner's group) the same rights as the group
- `chmod o= path/to/file` : remove all rights from others
- `chmod 644 path/to/file` : set rights using the octal notation
- `chmod -R a+rX path/to/directory` : recursively give all users read permissions to files. Also give eXecute permissions to files that have at least one execution permission and to all sub-directories

## Categories
files, users, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gchmod
