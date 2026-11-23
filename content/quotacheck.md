# quotacheck

Scan a filesystem for disk usage; create, check, and repair quota files. It is best to run quota check with quotas turned off to prevent damage or loss to quota files.

## Usage
- `sudo quotacheck --all` : check quotas on all mounted non-NFS filesystems
- `sudo quotacheck --force path/to/mount_point` : force check even if quotas are enabled (this can cause damage or loss to quota files)
- `sudo quotacheck --debug path/to/mount_point` : check quotas on a given filesystem in debug mode
- `sudo quotacheck --verbose path/to/mount_point` : check quotas on a given filesystem, displaying the progress
- `sudo quotacheck --user user path/to/mount_point` : check user quotas
- `sudo quotacheck --group group path/to/mount_point` : check group quotas

## Categories
users, disks, files

## Links
- More information: https://manned.org/quotacheck
- tldr page: https://tldr.inbrowser.app/pages/linux/quotacheck
