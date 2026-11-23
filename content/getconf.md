# getconf

Get configuration values from your Linux system.

## Usage
- `getconf -a` : list all configuration values available
- `getconf -a path/to/directory` : list the configuration values for a specific directory
- `getconf LONG_BIT` : check if the system is 32-bit or 64-bit
- `getconf CHILD_MAX` : check how many processes the current user can run at once
- `getconf -a | grep MAX` : list every configuration value and then find patterns with the `grep` command (i.e. every value with MAX in it)

## Categories
files

## Links
- More information: https://manned.org/getconf.1
- tldr page: https://tldr.inbrowser.app/pages/linux/getconf
