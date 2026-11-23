# edquota

Edit quotas for a user or group. By default it operates on all filesystems with quotas. Quota information is stored permanently in the `quota.user` and `quota.group` files in the root of the filesystem.

## Usage
- `edquota -u $(whoami)` : edit quota of the current user
- `sudo edquota -u username` : edit quota of a specific user
- `sudo edquota -g group` : edit quota for a group
- `sudo edquota -f filesystem` : restrict operations to a given filesystem (by default edquota operates on all filesystems with quotas)
- `sudo edquota -t` : edit the default grace period
- `sudo edquota -p reference_user destination_user1 destination_user2 ...` : duplicate a quota to other users

## Categories
users, disks

## Links
- More information: https://manned.org/edquota
- tldr page: https://tldr.inbrowser.app/pages/linux/edquota
