# php-coveralls

A PHP client for Coveralls.

## Usage
- `php-coveralls` : send coverage information to Coveralls
- `php-coveralls -r path/to/directory` : send coverage information to Coveralls for a specific directory
- `php-coveralls -c path/to/.coveralls.yml` : send coverage information to Coveralls with a specific config
- `php-coveralls -v` : send coverage information to Coveralls with verbose output
- `php-coveralls --exclude-no-stmt` : send coverage information to Coveralls excluding source files with no executable statements
- `php-coveralls -e test|dev|prod` : send coverage information to Coveralls with a specific environment name
- `php-coveralls -x path/to/first_clover.xml --coverage_clover path/to/second_clover.xml` : specify multiple Coverage Clover XML files to upload
- `php-coveralls -o path/to/coveralls-upload.json` : output the JSON that will be sent to Coveralls to a specific file

## Categories
files, text, network

## Links
- More information: https://php-coveralls.github.io/php-coveralls/#cli-options
- tldr page: https://tldr.inbrowser.app/pages/common/php-coveralls
