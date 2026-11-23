# zpool

Manage ZFS pools.

## Usage
- `zpool status` : show the configuration and status of all ZFS zpools
- `zpool scrub pool_name` : check a ZFS pool for errors (verifies the checksum of EVERY block). Very CPU and disk intensive
- `zpool import` : list zpools available for import
- `zpool import pool_name` : import a zpool
- `zpool export pool_name` : export a zpool (unmount all filesystems)
- `zpool history pool_name` : show the history of all pool operations
- `zpool create pool_name mirror disk1 disk2 mirror disk3 disk4` : create a mirrored pool
- `zpool add pool_name cache cache_disk` : add a cache (L2ARC) device to a zpool

## Links
- More information: https://manned.org/zpool
- tldr page: https://tldr.inbrowser.app/pages/common/zpool
