# phpstan

A PHP static analysis tool to discover bugs in code.

## Usage
- `phpstan analyse path/to/directory1 path/to/directory2 ...` : analyze one or more directories
- `phpstan analyse path/to/directory -c path/to/config` : analyze a directory using a configuration file
- `phpstan analyse path/to/directory -l 1..10` : analyze using a specific rule level (0-10, higher is stricter)
- `phpstan analyse path/to/directory -a path/to/autoload_file` : specify an autoload file to load before analyzing
- `phpstan analyse path/to/directory --memory-limit memory_limit` : specify a memory limit during analysis
- `phpstan analyse --help` : display available options for analysis

## Categories
files, monitoring

## Links
- More information: https://phpstan.org/user-guide/command-line-usage
- tldr page: https://tldr.inbrowser.app/pages/common/phpstan
