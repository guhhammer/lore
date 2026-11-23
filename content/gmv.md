# gmv

This command is an alias of GNU `mv`.

## Usage
- `mv path/to/source path/to/target` : rename a file or directory when the target is not an existing directory
- `mv path/to/source path/to/existing_directory` : move a file or directory into an existing directory
- `mv path/to/source1 path/to/source2 ... path/to/existing_directory` : move multiple files into an existing directory, keeping the filenames unchanged
- `mv -f path/to/source path/to/target` : do not prompt for confirmation before overwriting existing files
- `mv -i path/to/source path/to/target` : prompt for confirmation interactively before overwriting existing files, regardless of file permissions
- `mv -n path/to/source path/to/target` : do not overwrite existing files at the target
- `mv -v path/to/source path/to/target` : move files in verbose mode, showing files after they are moved
- `find /var/log -type f -name '*.log' -print0 | xargs -0 mv -t path/to/target_directory` : specify target directory so that you can use external tools to gather movable files

## Categories
files, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gmv
