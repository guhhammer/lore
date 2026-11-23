# repquota

Display a summary of existing file quotas for a filesystem.

## Usage
- `sudo repquota -a` : report stats for all quotas in use
- `sudo repquota -v filesystem` : report quota stats for all users, even those who aren't using any of their quota
- `repquota -u filesystem` : report on quotas for users only
- `sudo repquota -g filesystem` : report on quotas for groups only
- `sudo repquota -s filesystem` : report on used quota and limits in a human-readable format
- `sudo repquota -augs` : report on all quotas for users and groups in a human-readable format

## Categories
users, disks, desktop

## Links
- More information: https://manned.org/repquota
- tldr page: https://tldr.inbrowser.app/pages/linux/repquota
