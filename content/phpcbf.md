# phpcbf

Fix violations detected by phpcs.

## Usage
- `phpcbf path/to/directory` : fix issues in the specified directory (defaults to the PEAR standard)
- `phpcbf -i` : display a list of installed coding standards
- `phpcbf path/to/directory --standard standard` : specify a coding standard to validate against
- `phpcbf path/to/directory --extensions file_extension1,file_extension2,...` : specify comma-separated file extensions to include when sniffing
- `phpcbf path/to/directory --bootstrap path/to/file1,path/to/file2,...` : specify a comma-separated list of files to load before processing
- `phpcbf path/to/directory -l` : don't recurse into subdirectories

## Categories
files

## Links
- More information: https://github.com/squizlabs/PHP_CodeSniffer
- tldr page: https://tldr.inbrowser.app/pages/common/phpcbf
