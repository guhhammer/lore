# nwipe

Securely erase disks using a variety of recognized methods.

## Usage
- `sudo nwipe` : launch the interactive Ncurses interface
- `sudo nwipe --autonuke /dev/sdX /dev/sdY ...` : wipe specific devices using the default method (DoD 3-pass) without confirmation prompts
- `sudo nwipe -m zero|one|gutmann|dod522022m|... /dev/sdX` : wipe a specific disk using a custom erasing method
- `sudo nwipe --autonuke --autopoweroff /dev/sdX` : wipe a disk and power off the system upon completion
- `nwipe -V` : display version

## Categories
disks, users, network

## Links
- More information: https://manned.org/nwipe
- tldr page: https://tldr.inbrowser.app/pages/common/nwipe
- Wikipedia: https://en.wikipedia.org/wiki/Nwipe
