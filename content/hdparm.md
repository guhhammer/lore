# hdparm

Get and set SATA and IDE hard drive parameters.

## Usage
- `sudo hdparm -I /dev/device` : request the identification info of a given device
- `sudo hdparm -B /dev/device` : get the Advanced Power Management level
- `sudo hdparm -B 1 /dev/device` : set the Advanced Power Management value (values 1-127 permit spin-down, and values 128-254 do not)
- `sudo hdparm -C /dev/device` : display the device's current power mode status
- `sudo hdparm -y /dev/device` : force a drive to immediately enter standby mode (usually causes a drive to spin down)
- `sudo hdparm -S standby_timeout device` : put the drive into idle (low-power) mode, also setting its standby timeout
- `sudo hdparm -tT device` : test the read speed of a specific device

## Categories
hardware, users, disks

## Links
- More information: https://manned.org/hdparm
- tldr page: https://tldr.inbrowser.app/pages/linux/hdparm
- Wikipedia: https://en.wikipedia.org/wiki/Hdparm
