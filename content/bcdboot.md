# bcdboot

Configure or repair boot files.

## Usage
- `bcdboot C:\Windows` : initialize the system partition by using BCD files from the source Windows folder
- `bcdboot C:\Windows /v` : enable verbose mode
- `bcdboot C:\Windows /s S:` : specify the volume letter of the system partition
- `bcdboot C:\Windows /l en-us` : specify a locale
- `bcdboot C:\Windows /s S: /f UEFI|BIOS|ALL` : specify a firmware type while copying the boot files to a specified volume

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
system, disks, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/bcdboot
