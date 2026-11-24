# mimikatz crypto

Manipulate Windows cryptographic services and certificates, allowing you to list and export certificates and keys, even those marked as non-exportable. It generally requires elevated privileges, especially when accessing system keys.

## Usage
- `mimikatz "crypto::providers"` : list cryptographic providers
- `mimikatz "crypto::capi"` : list keys in a cryptographic provider
- `mimikatz "crypto::certificates /export"` : export certificates and keys

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
system, windows

## Links
- More information: https://github.com/gentilkiwi/mimikatz
- tldr page: https://tldr.inbrowser.app/pages/windows/mimikatz-crypto
- Wikipedia: https://en.wikipedia.org/wiki/Mimikatz
