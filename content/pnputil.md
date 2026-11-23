# pnputil

Manage the driver store and driver packages on Windows. Requires an elevated Command Prompt.

## Usage
- `pnputil /enum-drivers` : list all installed driver packages
- `pnputil /add-driver path\to\driver.inf` : install a driver package from an INF file
- `pnputil /add-driver path\to\folder\*.inf /subdirs` : add and install all INF files in a folder (including subdirectories)
- `pnputil /delete-driver oemXX.inf` : delete a driver package using its published name
- `pnputil /add-driver oemXX.inf` : install a driver package using its published name
- `pnputil /export-driver * path\to\folder` : export all driver packages to a folder
- `pnputil /enum-devices` : list all devices

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
packages, files, shell, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/pnputil
