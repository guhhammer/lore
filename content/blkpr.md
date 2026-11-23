# blkpr

Register, reserve, release, preempt, and clear persistent reservations on a block device that supports Persistent Reservations.

## Usage
- `blkpr -c register -k reservation_key path/to/device` : register (command) a new reservation with a given key on a given device
- `blkpr -c reserve -k reservation_key -t exclusive-access path/to/device` : set the type of an existing reservation to exclusive access
- `blkpr -c preempt -K old_key -k new_key -t write-exclusive path/to/device` : preempt the existing reservation with a given key and replace it with a new reservation
- `blkpr -c release -k reservation_key -t reservation_type path/to/device` : release a reservation with a given key and type on a given device
- `blkpr -c clear -k key path/to/device` : clear all reservations from a given device

## Categories
hardware, files, disks

## Links
- More information: https://manned.org/blkpr
- tldr page: https://tldr.inbrowser.app/pages/linux/blkpr
