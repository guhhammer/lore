# tlp-stat

Generate TLP status reports.

## Usage
- `sudo tlp-stat` : generate status report with configuration and all active settings
- `sudo tlp-stat --battery|disk|processor|graphics|pcie|rfkill|usb` : show information about various devices
- `sudo tlp-stat -v --battery|processor|pcie|usb` : show verbose information about devices that support verbosity
- `sudo tlp-stat -c` : show configuration
- `sudo tlp-stat -P` : monitor power supply `udev` [ev]ents
- `sudo tlp-stat --psup` : show power [sup]ply diagnostics
- `sudo tlp-stat -t` : show temperatures and fan speed
- `sudo tlp-stat -s` : show general system information

## See also
- tlp

## Categories
users, hardware

## Links
- More information: https://linrunner.de/tlp/usage/tlp-stat
- tldr page: https://tldr.inbrowser.app/pages/linux/tlp-stat
