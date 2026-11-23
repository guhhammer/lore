# wine reg add

Add new keys and values to the Wine registry.

## Usage
- `wine reg add 'HKEY_CURRENT_USER\path\to\key'` : add a new registry key
- `wine reg add 'HKEY_CURRENT_USER\path\to\key' /v value_name /d data` : add a value with specific data under a key
- `wine reg add 'HKEY_CURRENT_USER\path\to\key' /v value_name /t REG_SZ|REG_DWORD|REG_QWORD|REG_BINARY|REG_MULTI_SZ|REG_EXPAND_SZ|REG_NONE /d data` : add a value with a specific data type (defaults to `REG_SZ`)
- `wine reg add 'HKEY_CURRENT_USER\path\to\key' /ve /d data` : set the default (unnamed) value of a key
- `wine reg add 'HKEY_CURRENT_USER\path\to\key' /v value_name /t REG_MULTI_SZ /s # /d 'string1#string2#string3'` : add a `REG_MULTI_SZ` value, specifying the string separator character
- `wine reg add 'HKEY_CURRENT_USER\path\to\key' /v value_name /d data /f` : forcefully overwrite an existing value without prompting for confirmation
- `wine reg add /?` : display help

## Categories
files

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/wine-reg-add
