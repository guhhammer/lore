# mimikatz lsadump

Dump secrets from the Windows Local Security Authority (LSA). Requires SYSTEM privileges.

## Usage
- `mimikatz "lsadump::sam"` : dump SAM hashes
- `mimikatz "lsadump::secrets"` : dump secrets from the SECURITY hive
- `mimikatz "lsadump::cache"` : dump cached domain credentials

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
windows

## Links
- More information: https://github.com/gentilkiwi/mimikatz
- tldr page: https://tldr.inbrowser.app/pages/windows/mimikatz-lsadump
