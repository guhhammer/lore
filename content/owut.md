# owut

OpenWrt Upgrade Tool for automating builds and installs via Attended SysUpgrade (ASU).

## Usage
- `owut versions` : list available OpenWrt versions for the current device
- `owut list` : list user installed or removed packages
- `owut check` : check for updates without performing an upgrade
- `owut upgrade` : automatically build, download, verify, and install the latest available firmware
- `owut upgrade -V version -a "htop vim"` : upgrade to a specific version and include additional packages
- `owut download -V 25.12.0` : download and verify a specific image without installing it
- `owut install` : verify the latest downloaded image and install it (device will reboot)

## Categories
packages, network, hardware

## Links
- More information: https://openwrt.org/docs/guide-user/installation/sysupgrade.owut
- tldr page: https://tldr.inbrowser.app/pages/linux/owut
