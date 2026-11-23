# sfdk emulator remove

The examples for this command have been moved together with `sfdk emulator install`.

## Usage
- `install path/to/source_file1 path/to/source_file2 ... path/to/destination` : copy files to the destination
- `install -o user path/to/source_file1 path/to/source_file2 ... path/to/destination` : copy files to the destination, setting their ownership
- `install -g user path/to/source_file1 path/to/source_file2 ... path/to/destination` : copy files to the destination, setting their group ownership
- `install -m +x path/to/source_file1 path/to/source_file2 ... path/to/destination` : copy files to the destination, setting their `mode`
- `install -p path/to/source_file1 path/to/source_file2 ... path/to/destination` : copy files and apply access/modification times of source to the destination
- `install -D path/to/source_file1 path/to/source_file2 ... path/to/destination` : copy files and create the directories at the destination if they don't exist

## Categories
files, packages, users

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/sfdk-emulator-remove
