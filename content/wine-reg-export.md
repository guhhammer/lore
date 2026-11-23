# wine reg export

Export keys, values, and data from the Wine registry to a `.reg` file.

## Usage
- `wine reg export 'HKEY_CURRENT_USER\path\to\key' path/to/file.reg` : export a registry key and its subkeys to a file
- `wine reg export HKEY_CURRENT_USER path/to/file.reg` : export an entire registry hive to a file
- `wine reg export 'HKEY_CURRENT_USER\path\to\key' path/to/file.reg /y` : overwrite the output file if it already exists, without prompting
- `wine reg export /?` : display help

## Categories
files

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/wine-reg-export
