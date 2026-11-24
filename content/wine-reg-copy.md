# wine reg copy

Copy a registry key to a new location within the Wine registry.

## Usage
- `wine reg copy 'HKEY_CURRENT_USER\path\to\source_key' 'HKEY_CURRENT_USER\path\to\destination_key'` : copy a registry key to a new location
- `wine reg copy 'HKEY_CURRENT_USER\path\to\source_key' 'HKEY_CURRENT_USER\path\to\destination_key' /s` : recursively copy a key together with all its subkeys and values
- `wine reg copy 'HKEY_CURRENT_USER\path\to\source_key' 'HKEY_CURRENT_USER\path\to\destination_key' /f` : forcefully overwrite the destination without prompting for confirmation
- `wine reg copy 'HKEY_CURRENT_USER\path\to\source_key' 'HKEY_CURRENT_USER\path\to\destination_key' /s /f` : recursively and forcefully copy a key, overwriting the destination
- `wine reg copy /?` : display help

## Categories
files

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/wine-reg-copy
- Wikipedia: https://en.wikipedia.org/wiki/Wine_(software)
