# phpdismod

Disable PHP extensions on Debian-based OSes.

## Usage
- `sudo phpdismod json` : disable the JSON extension for every SAPI of every PHP version
- `sudo phpdismod -v 7.3 -s cli json` : disable the JSON extension for PHP 7.3 with the cli SAPI

## Categories
text

## Links
- More information: https://salsa.debian.org/php-team/php-defaults
- tldr page: https://tldr.inbrowser.app/pages/linux/phpdismod
