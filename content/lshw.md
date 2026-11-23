# lshw

List detailed information about hardware configurations as root user.

## Usage
- `sudo lshw -X` : launch the X11 GUI (if available)
- `sudo lshw -short` : list all hardware in tabular format
- `sudo lshw -c disk -c storage -short` : list multiple class of hardware (all disks and storage controllers) in tabular format
- `sudo lshw -c network -html|xml|json >interfaces.html|.xml|.json` : save all network interfaces to an HTML/XML/JSON file
- `sudo lshw -c network -sanitize` : list network interfaces without revealing sensitive information (IP addresses, serial numbers, etc.)
- `sudo lshw -c system|bridge|memory|processor|address|storage|disk|tape|bus|network|display|input|printer|multimedia|communication|power|volume|generic` : list a particular class of hardware

## See also
- inxi
- hwinfo
- dmidecode

## Categories
users, network, text

## Links
- More information: https://ezix.org/project/wiki/HardwareLiSter
- tldr page: https://tldr.inbrowser.app/pages/linux/lshw
