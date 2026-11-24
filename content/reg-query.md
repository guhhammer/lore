# reg query

Display the values of keys and subkeys in the registry.

## Usage
- `reg query key_name` : display all values of a key
- `reg query key_name /v value` : display a specific value of a key
- `reg query key_name /s` : display all values of a key and its subkeys
- `reg query key_name /f "query_pattern"` : search for keys and values matching a specific pattern
- `reg query key_name /t REG_SZ|MULTI_SZ|EXPAND_SZ|DWORD|BINARY|NONE` : display a value of a key matching a specified data type
- `reg query key_name /d` : only search in data
- `reg query key_name /f "query_pattern" /k` : only search in key names
- `reg query key_name /c /e` : case-sensitively search for an exact match

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
databases, desktop, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/reg-query
- Wikipedia: https://en.wikipedia.org/wiki/Windows_Registry
