# phpquery

PHP extension manager for Debian-based OSes.

## Usage
- `sudo phpquery -V` : list available PHP versions
- `sudo phpquery -v 7.3 -S` : list available SAPIs for PHP 7.3
- `sudo phpquery -v 7.3 -s cli -M` : list enabled extensions for PHP 7.3 with the cli SAPI
- `sudo phpquery -v 7.3 -s apache2 -m json` : check if the JSON extension is enabled for PHP 7.3 with the apache2 SAPI

## Categories
users

## Links
- More information: https://code.google.com/archive/p/phpquery/wikis/CommandLineInterface.wiki
- tldr page: https://tldr.inbrowser.app/pages/linux/phpquery
