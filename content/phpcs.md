# phpcs

Tokenize PHP, JavaScript, and CSS files to detect violations of a defined set of coding standards.

## Usage
- `phpcs path/to/directory` : sniff the specified directory for issues (defaults to the PEAR standard)
- `phpcs -i` : display a list of installed coding standards
- `phpcs path/to/directory --standard standard` : specify a coding standard to validate against
- `phpcs path/to/directory --extensions file_extension1,file_extension2,...` : specify comma-separated file extensions to include when sniffing
- `phpcs path/to/directory --report format` : specify the format of the output report (e.g. `full`, `xml`, `json`, `summary`)
- `phpcs path/to/directory --config-set key value` : set configuration variables to be used during the process
- `phpcs path/to/directory --bootstrap path/to/file1,path/to/file2,...` : specify a comma-separated list of files to load before processing
- `phpcs path/to/directory -l` : don't recurse into subdirectories

## Categories
files

## Links
- More information: https://github.com/squizlabs/PHP_CodeSniffer
- tldr page: https://tldr.inbrowser.app/pages/common/phpcs
