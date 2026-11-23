# duplicacy

A lock-free deduplication cloud backup tool.

## Usage
- `duplicacy init -e snapshot_id sftp://user@192.168.2.100/path/to/storage/` : use current directory as the repository, initialize an SFTP storage, and encrypt the storage with a password
- `duplicacy backup` : save a snapshot of the repository to the default storage
- `duplicacy list` : list snapshots of current repository
- `duplicacy restore -r revision` : restore the repository to a previously saved snapshot
- `duplicacy check` : check the integrity of snapshots
- `duplicacy add storage_name snapshot_id storage_url` : add another storage to be used for the existing repository
- `duplicacy prune -r revision` : prune a specific revision of snapshot
- `duplicacy prune -keep n:m` : prune revisions, keeping one revision every `n` days for all revisions older than `m` days

## Categories
packages, cloud

## Links
- More information: https://github.com/gilbertchen/duplicacy/wiki
- tldr page: https://tldr.inbrowser.app/pages/common/duplicacy
