# dconf write

Write key values in dconf databases.

## Usage
- `dconf write /path/to/key "value"` : write a specific key value
- `dconf write /path/to/key "'string'"` : write a specific string key value
- `dconf write /path/to/key "5"` : write a specific integer key value
- `dconf write /path/to/key "true|false"` : write a specific boolean key value
- `dconf write /path/to/key "['first', 'second', ...]"` : write a specific array key value
- `dconf write /path/to/key "@as []"` : write a specific empty array key value

## Categories
files

## Links
- More information: https://manned.org/dconf
- tldr page: https://tldr.inbrowser.app/pages/linux/dconf-write
