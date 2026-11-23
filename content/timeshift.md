# timeshift

System restore utility.

## Usage
- `sudo timeshift --list` : list snapshots
- `sudo timeshift --check` : create a new snapshot (if scheduled)
- `sudo timeshift --create` : create a new snapshot (even if not scheduled)
- `sudo timeshift --restore` : restore a snapshot (selecting which snapshot to restore interactively)
- `sudo timeshift --restore --snapshot 'snapshot'` : restore a specific snapshot
- `sudo timeshift --delete --snapshot 'snapshot'` : delete a specific snapshot

## Categories
users

## Links
- More information: https://manned.org/timeshift
- tldr page: https://tldr.inbrowser.app/pages/linux/timeshift
