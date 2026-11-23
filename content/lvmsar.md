# lvmsar

LVM system activity reporter. Not supported under LVM2; prefer `dmstats`.

## Usage
- `lvmsar` : run the legacy reporter (LVM1 systems only)
- `dmstats report /dev/mapper/device` : report I/O statistics for a device using device-mapper stats
- `dmstats list /dev/mapper/device` : list statistics regions for a device

## Categories
hardware, disks

## Links
- More information: https://manned.org/lvmsar
- tldr page: https://tldr.inbrowser.app/pages/linux/lvmsar
