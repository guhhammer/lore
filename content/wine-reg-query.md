# wine reg query

Display keys, values, and data in the Wine registry.

## Usage
- `wine reg query 'HKEY_CURRENT_USER\path\to\key'` : display all values directly under a key
- `wine reg query 'HKEY_CURRENT_USER\path\to\key' /v value_name` : display a specific value of a key
- `wine reg query 'HKEY_CURRENT_USER\path\to\key' /ve` : display the default (unnamed) value of a key
- `wine reg query 'HKEY_CURRENT_USER\path\to\key' /s` : recursively display a key together with all its subkeys and values
- `wine reg query /?` : display help

## Categories
databases, desktop, files

## Links
- More information: https://gitlab.winehq.org/wine/wine/-/wikis/Commands
- tldr page: https://tldr.inbrowser.app/pages/linux/wine-reg-query
- Wikipedia: https://en.wikipedia.org/wiki/Wine_(software)
