# lvmconfig

Display and manipulate LVM configuration information.

## Usage
- `lvmconfig --typeconfig current --mergedconfig` : display the effective configuration in use (after merging all config sources)
- `lvmconfig --typeconfig diff` : show only settings that differ from their defaults
- `lvmconfig -l` : list all configuration keys
- `lvmconfig --typeconfig default --withcomments --withspaces` : print the default configuration with full comments and extra spacing
- `lvmconfig --mergedconfig --validate` : validate the full merged configuration and report errors
- `lvmconfig --typeconfig current -f path/to/output.conf` : write the current effective configuration to a file

## Categories
desktop, disks

## Links
- More information: https://manned.org/lvmconfig
- tldr page: https://tldr.inbrowser.app/pages/linux/lvmconfig
