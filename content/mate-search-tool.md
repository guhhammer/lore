# mate-search-tool

Search files in MATE desktop environment.

## Usage
- `mate-search-tool --named=string --path=path/to/directory` : search files containing a specific string in their name in a specific directory
- `mate-search-tool --start --named=string --path=path/to/directory` : search files without waiting a user confirmation
- `mate-search-tool --start --regex=string --path=path/to/directory` : search files with name matching a specific `regex`
- `mate-search-tool --start --named=string --path=path/to/directory --sortby=name|folder|size|type|date` : set a sorting order in search results
- `mate-search-tool --start --named=string --path=path/to/directory --descending` : set a descending sorting order
- `mate-search-tool --start --user|group=value --path=path/to/directory` : search files owned by a specific user/group

## Categories
files, text, users

## Links
- More information: https://manned.org/mate-search-tool
- tldr page: https://tldr.inbrowser.app/pages/linux/mate-search-tool
