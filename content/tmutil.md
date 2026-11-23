# tmutil

Utility for managing Time Machine backups.

## Usage
- `sudo tmutil setdestination path/to/disk_mount_point` : set an HFS+ drive as the backup destination
- `sudo tmutil setdestination "protocol://user:password@host/share"` : set an APF share or SMB share as the backup destination
- `sudo tmutil setdestination -a destination` : append the given destination to the list of destinations
- `sudo tmutil enable` : enable automatic backups
- `sudo tmutil disable` : disable automatic backups
- `sudo tmutil startbackup` : start a backup, if one is not running already, and release control of the shell
- `sudo tmutil startbackup -b` : start a backup and block until the backup is finished
- `sudo tmutil stopbackup` : stop a backup

## Categories
users, network, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/tmutil.8.html
- tldr page: https://tldr.inbrowser.app/pages/osx/tmutil
