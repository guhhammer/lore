# wine reg delete

Delete keys and values from the Wine registry.

## Usage
- `wine reg delete 'HKEY_CURRENT_USER\path\to\key'` : delete a registry key and all of its values
- `wine reg delete 'HKEY_CURRENT_USER\path\to\key' /v value_name` : delete a specific value under a key
- `wine reg delete 'HKEY_CURRENT_USER\path\to\key' /ve` : delete the default (unnamed) value of a key
- `wine reg delete 'HKEY_CURRENT_USER\path\to\key' /va` : delete all values under a key without removing the key itself
- `wine reg delete 'HKEY_CURRENT_USER\path\to\key' /f` : forcefully delete without prompting for confirmation
- `wine reg delete /?` : display help

## Categories
files

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/wine-reg-delete
