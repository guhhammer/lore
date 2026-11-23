# sfc

Scan the integrity of Windows system files.

## Usage
- `sfc /scannow` : scan all system files and, if possible, repair any problems
- `sfc /verifyonly` : scan all system files without attempting to repair any
- `sfc /scanfile=path\to\file` : scan a specific file and, if possible, repair any problems
- `sfc /verifyfile=path\to\file` : scan a specific file without attempting to repair it
- `sfc /offbootdir=path\to\directory` : when repairing offline, specify the boot directory
- `sfc /offwindir=path\to\directory` : when repairing offline, specify the Windows directory
- `sfc` : display help

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
files, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/sfc
