# mimikatz sekurlsa

Extract credentials and secrets from memory. Requires debug privileges.

## Usage
- `mimikatz "sekurlsa::logonpasswords"` : extract plaintext passwords
- `mimikatz "sekurlsa::tickets"` : list Kerberos tickets in memory
- `mimikatz "sekurlsa::secrets"` : dump LSA secrets

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
archives, monitoring, development, windows

## Links
- More information: https://github.com/gentilkiwi/mimikatz
- tldr page: https://tldr.inbrowser.app/pages/windows/mimikatz-sekurlsa
