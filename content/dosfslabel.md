# dosfslabel

Set or get MS-DOS filesystem label or volume ID.

## Usage
- `fatlabel [OPTIONS] DEVICE [NEW]` : the general form, as the manual gives it
- `dosfslabel -i` : switch to volume ID mode
- `dosfslabel -r` : remove label in label mode or generate new ID in volume ID mode
- `dosfslabel -c PAGE` : use DOS codepage PAGE to encode/decode label
- `dosfslabel -h` : display a help message and terminate

## Categories
disks
