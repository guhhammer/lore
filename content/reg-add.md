# reg add

Add new keys and their values to the registry.

## Usage
- `reg add key_name` : add a new registry key
- `reg add key_name /v value` : add a new value under a specific key
- `reg add key_name /d data` : add a new value with specific data
- `reg add key_name /t REG_SZ|MULTI_SZ|DWORD_BIG_ENDIAN|DWORD|BINARY|DWORD_LITTLE_ENDIAN|LINK|FULL_RESOURCE_DESCRIPTOR|EXPAND_SZ` : add a new value to a key with a specific data type
- `reg add key_name /f` : forcefully (without a prompt) overwrite the existing registry value

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/reg-add
- Wikipedia: https://en.wikipedia.org/wiki/Windows_Registry
