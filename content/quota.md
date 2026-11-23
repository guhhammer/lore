# quota

Display users' disk space usage and allocated limits.

## Usage
- `quota -s` : show disk quotas in human-readable units for the current user
- `quota -v` : display disk quotas verbosely (also display quotas on filesystems where no storage is allocated)
- `quota -q` : display disk quotas quietly (only display quotas on filesystems where usage is over quota)
- `quota -g` : print quotas for the groups of which the current user is a member
- `sudo quota -u username` : show disk quotas for another user

## Categories
users, desktop, disks

## Links
- More information: https://manned.org/quota
- tldr page: https://tldr.inbrowser.app/pages/common/quota
