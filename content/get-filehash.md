# get-filehash

Calculate cryptographic checksums for a file. Note: This command can only be used through PowerShell.

## Usage
- `Get-FileHash path\to\file` : calculate the SHA256 checksum for a file
- `Get-FileHash path\to\file -Algorithm SHA1|SHA256|SHA384|SHA512|MD5` : calculate the checksum for a file using a specified algorithm
- `Get-FileHash path\to\file -Algorithm MACTripleDES|RIPEMD160` : calculate the checksum for a file using a deprecated algorithm (no longer available in PowerShell 6 or later)
- `(Get-FileHash path\to\file -Algorithm SHA1|SHA256|SHA384|SHA512|MD5).Hash -eq "known_hash"` : check a known checksum of a file

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, security, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/get-filehash
