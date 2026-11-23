# phploc

Quickly measure the size and analyzing the structure of a PHP project.

## Usage
- `phploc path/to/directory` : analyze a directory and print the result
- `phploc path/to/directory --names 'path/to/file1,path/to/file2,...'` : include only specific files from a comma-separated list (`glob`s are allowed)
- `phploc path/to/directory --names-exclude 'path/to/file1,path/to/file2,...'` : exclude specific files from a comma-separated list (`glob`s are allowed)
- `phploc path/to/directory --exclude path/to/exclude_directory` : exclude a specific directory from analysis
- `phploc path/to/directory --log-csv path/to/file` : log the results to a specific CSV file
- `phploc path/to/directory --log-xml path/to/file` : log the results to a specific XML file
- `phploc path/to/directory --count-tests` : count PHPUnit test case classes and test methods

## Categories
files, development, monitoring

## Links
- More information: https://github.com/sebastianbergmann/phploc
- tldr page: https://tldr.inbrowser.app/pages/common/phploc
