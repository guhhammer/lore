# uwfmgr

Unified Write Filter (UWF). Protect drives by redirecting any writes to the drive to a virtual overlay. Writes are discarded upon reboot unless committed by default.

## Usage
- `uwfmgr get-config` : get the current status
- `uwfmgr volume protect drive_letter:` : set a drive as protected
- `uwfmgr volume unprotect drive_letter:` : remove a drive from protection list
- `uwfmgr filter enable|disable` : enable or disable protection (Applies after reboot)
- `uwfmgr file commit drive_letter:\path\to\file` : commit changes of a file on protected drive
- `uwfmgr file commit-delete drive_letter:\path\to\file` : commit deletion of a file on protected drive

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
disks, files, system, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/uwfmgr
