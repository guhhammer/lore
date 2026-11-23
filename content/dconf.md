# dconf

Manage dconf databases.

## Usage
- `dconf read /path/to/key` : print a specific key value
- `dconf list /path/to/directory/` : print a specific path sub-directories and sub-keys
- `dconf write /path/to/key "value"` : write a specific key value
- `dconf reset /path/to/key` : reset a specific key value
- `dconf watch /path/to/key|/path/to/directory/` : watch a specific key/directory for changes
- `dconf dump /path/to/directory/` : dump a specific directory in INI file format

## See also
- gsettings

## Categories
files

## Links
- More information: https://manned.org/dconf
- tldr page: https://tldr.inbrowser.app/pages/linux/dconf
