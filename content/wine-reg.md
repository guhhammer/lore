# wine reg

Manage keys and their values in the Wine registry. Some subcommands such as `add`, `delete`, `query`, etc. have their own usage documentation.

## Usage
- `wine reg add 'HKEY_CURRENT_USER\path\to\key' /v value_name /d data /f` : add or overwrite a value under a key (creating the key if needed)
- `wine reg query 'HKEY_CURRENT_USER\path\to\key'` : display the values stored under a key
- `wine reg delete 'HKEY_CURRENT_USER\path\to\key' /v value_name /f` : delete a value from a key
- `wine reg copy 'HKEY_CURRENT_USER\path\to\source_key' 'HKEY_CURRENT_USER\path\to\destination_key' /s` : copy a key and all its subkeys to a new location
- `wine reg export 'HKEY_CURRENT_USER\path\to\key' path/to/file.reg` : export a key and its subkeys to a `.reg` file
- `wine reg import path/to/file.reg` : import registry entries from a `.reg` file
- `wine reg /?` : display help

## Categories
files, databases, documents

## Links
- More information: https://gitlab.winehq.org/wine/wine/-/wikis/Commands
- tldr page: https://tldr.inbrowser.app/pages/linux/wine-reg
- Wikipedia: https://en.wikipedia.org/wiki/Wine_(software)
