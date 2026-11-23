# lvmpersist

Manage persistent reservations (PR) on block devices or all PVs in a volume group.

## Usage
- `lvmpersist start --ourkey 0x1234abcd --vg vg_name` : start PR on all PVs in a VG with a local key (exclusive access by default)
- `lvmpersist start --ourkey 0x1234abcd --access sh --vg vg_name` : start PR for a shared VG (allow multiple hosts)
- `lvmpersist stop --ourkey 0x1234abcd --vg vg_name` : stop PR on a VG and unregister the local key
- `lvmpersist start --ourkey 0xmy_key --removekey 0xother_key --vg vg_name` : take over a local VG by preempting another host while starting PR
- `lvmpersist remove --ourkey 0xmy_key --removekey 0xother_key --vg vg_name` : remove another host's key from a shared VG
- `lvmpersist read --vg vg_name` : show registered keys and reservations for a VG
- `lvmpersist start --ourkey 0x1234abcd --device /dev/sdX --device /dev/mapper/mpathY` : operate on specific devices instead of a VG

## Categories
hardware, network, disks

## Links
- More information: https://manned.org/lvmpersist
- tldr page: https://tldr.inbrowser.app/pages/linux/lvmpersist
