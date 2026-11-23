# cleanmgr

Clear unnecessary files from the computer's hard disk. Note: This command is deprecated in newer Windows versions, use "Storage Sense" instead.

## Usage
- `cleanmgr /d C` : open Disk Cleanup for a specific drive
- `cleanmgr /d C /lowdisk` : open Disk Cleanup with all options selected by default
- `cleanmgr /d C /verylowdisk` : clean up all files automatically without user prompts
- `cleanmgr /sageset:profile_number` : configure which files to clean and save the settings to a specific profile
- `cleanmgr /sagerun:profile_number` : run Disk Cleanup using a previously configured profile
- `cleanmgr /autoclean` : clean up files left after a Windows upgrade
- `cleanmgr /setup` : clean up files from a previous Windows installation
- `cleanmgr /?` : display help

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
disks, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/cleanmgr
