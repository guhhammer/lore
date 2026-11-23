# mimikatz dpapi

Interact with the Windows Data Protection API (DPAPI).

## Usage
- `mimikatz "dpapi::masterkey /list"` : list master keys
- `mimikatz "dpapi::blob /in:blob_file.bin"` : decrypt a DPAPI blob
- `mimikatz "dpapi::chrome /in:Login Data"` : retrieve Chrome credentials using DPAPI

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
windows

## Links
- More information: https://github.com/gentilkiwi/mimikatz
- tldr page: https://tldr.inbrowser.app/pages/windows/mimikatz-dpapi
