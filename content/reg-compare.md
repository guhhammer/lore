# reg compare

Compare keys and their values in the registry.

## Usage
- `reg compare key_name1 key_name2` : compare all values under a specific key with another key
- `reg compare key_name1 key_name2 /v value` : compare a specific value under two keys
- `reg compare key_name1 key_name2 /s` : compare all subkeys and values for two keys
- `reg compare key_name1 key_name2 /os` : only output the matches (same) between the specified keys
- `reg compare key_name1 key_name2 /oa` : output the differences and matches (all) between the specified keys
- `reg compare key_name1 key_name2 /on` : compare two keys, outputting nothing

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/reg-compare
- Wikipedia: https://en.wikipedia.org/wiki/Windows_Registry
