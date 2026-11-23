# rusnapshot

BTRFS snapshotting utility written in Rust.

## Usage
- `sudo rusnapshot -c path/to/config.toml --cr` : create a snapshot using a configuration file
- `sudo rusnapshot -c path/to/config.toml -l` : list created snapshots
- `sudo rusnapshot -c path/to/config.toml --del --id snapshot_id` : delete a snapshot by ID or the name of the snapshot
- `sudo rusnapshot -c path/to/config.toml -l -k 0 --clean --kind hourly` : delete all `hourly` snapshots
- `sudo rusnapshot -c path/to/config.toml --cr -r` : create a read-write snapshot
- `sudo rusnapshot -c path/to/config.toml --id snapshot_id -r` : restore a snapshot

## Categories
files, users

## Links
- More information: https://github.com/Edu4rdSHL/rusnapshot
- tldr page: https://tldr.inbrowser.app/pages/linux/rusnapshot
