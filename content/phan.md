# phan

A static analysis tool for PHP.

## Usage
- `phan --init` : generate a `.phan/config.php` in the current directory
- `phan --init --init-level 1..5` : generate a Phan configuration file using a specific level (1 being strictest to 5 being the least strict)
- `phan` : analyze the current directory
- `phan --directory path/to/directory1 --directory path/to/directory2 ...` : analyze one or more directories
- `phan --config-file path/to/config.php` : specify a configuration file (defaults to `.phan/config.php`)
- `phan --output-mode text|verbose|json|csv|codeclimate|checkstyle|pylint|html` : specify the output mode
- `phan --processes number_of_processes` : specify the number of parallel processes

## Categories
files, text

## Links
- More information: https://github.com/phan/phan
- tldr page: https://tldr.inbrowser.app/pages/common/phan
