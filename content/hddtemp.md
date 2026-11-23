# hddtemp

Display temperature of HDD via S.M.A.R.T.

## Usage
- `hddtemp type:/dev/sdX` : display temperature of a specific drive
- `hddtemp SATA:/dev/sda` : display temperature of a SATA drive assigned to `sda`
- `hddtemp -S nseconds type:/dev/sdX` : log temperatures to syslog every `n` seconds
- `hddtemp -n type:/dev/sdX` : print only numeric value of temperature without unit
- `hddtemp -u C|F type:/dev/sdX` : define the unit used to denote temperature
- `hddtemp -w type:/dev/sdX` : wake ATA drive before attempting to read temperature
- `hddtemp -D type:/dev/sdX` : enter debug mode to show S.M.A.R.T. fields and their values
- `hddtemp -q type:/dev/sdX` : suppress compatibility check for drive types

## Categories
desktop, disks

## Links
- More information: https://manned.org/hddtemp
- tldr page: https://tldr.inbrowser.app/pages/linux/hddtemp
