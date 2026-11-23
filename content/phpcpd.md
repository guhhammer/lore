# phpcpd

A copy and paste detector for PHP code.

## Usage
- `phpcpd path/to/file_or_directory` : analyze duplicated code for a specific file or directory
- `phpcpd --fuzzy path/to/file_or_directory` : analyze using fuzzy matching for variable names
- `phpcpd --min-lines number_of_lines path/to/file_or_directory` : specify a minimum number of identical lines (defaults to 5)
- `phpcpd --min-tokens number_of_tokens path/to/file_or_directory` : specify a minimum number of identical tokens (defaults to 70)
- `phpcpd --exclude path/to/excluded_directory path/to/file_or_directory` : exclude a directory from analysis (must be relative to the source)
- `phpcpd --log-pmd path/to/log_file path/to/file_or_directory` : output the results to a PHP-CPD XML file

## Categories
files, text

## Links
- More information: https://github.com/sebastianbergmann/phpcpd
- tldr page: https://tldr.inbrowser.app/pages/common/phpcpd
