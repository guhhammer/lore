# repair-bde

Attempt to repair or decrypt a damaged BitLocker-encrypted volume.

## Usage
- `repair-bde C:` : attempt to repair a specified volume
- `repair-bde C: D:` : attempt to repair a specified volume and output to another volume
- `repair-bde C: -RecoveryKey path\to\file.bek` : attempt to repair a specified volume using the provided recovery key file
- `repair-bde C: -RecoveryPassword password` : attempt to repair a specified volume using the provided numerical recovery password
- `repair-bde C: -Password password` : attempt to repair a specified volume using the provided password
- `repair-bde C: -KeyPackage path\to\directory` : attempt to repair a specified volume using the provided key package
- `repair-bde C: -LogFile path\to\file` : log all output to a specific file
- `repair-bde /?` : display help

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
disks, files, users, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/repair-bde
