# in

This shell keyword is used with `for` to separate the iterator and items to iterate.

## Usage
- `for variable; do echo $variable; done` : iterate through command line parameters
- `for variable in item1 item2 ...; do echo "Loop is executed"; done` : execute the given commands for each of the specified items
- `for variable in {from..to..step}; do echo "Loop is executed"; done` : iterate over a given range of numbers
- `for variable in path/to/file1 path/to/file2 ...; do echo "Loop is executed"; done` : iterate over a given list of files
- `for variable in path/to/directory1/ path/to/directory2/ ...; do echo "Loop is executed"; done` : iterate over a given list of directories
- `for variable in */; do (cd "$variable" || continue; echo "Loop is executed") done` : perform a given command in every directory
- `for ((i=0; i<10; i++)); do echo $i; done` : use a C-style `for` loop

## Categories
files, shell

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/in
