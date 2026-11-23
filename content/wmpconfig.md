# wmpconfig

Manage network sharing for Windows Media Player (legacy). Note: This command requires administrator privileges.

## Usage
- `wmpconfig HMEOn|HMEOff` : turn network sharing on or off
- `wmpconfig DisableHMEDevice mac_address` : stop a specific network device from sharing to this computer using its MAC address
- `wmpconfig RemoveHMEDevice mac_address` : remove a network device from sharing to this computer
- `wmpconfig RestoreHMEDevice mac_address` : restore a network device from sharing to this computer
- `wmpconfig SetDVDParentalLevel 3` : set the parental control level for playing DVD content

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## See also
- wmpnscfg

## Categories
network, hardware, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/wmpconfig
