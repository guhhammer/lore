# networksetup

Configuration tool for Network System Preferences.

## Usage
- `networksetup -listallnetworkservices` : list available network service providers (Ethernet, Wi-Fi, Bluetooth, etc.)
- `networksetup -getinfo "Wi-Fi"` : show network settings for a particular networking device
- `networksetup -getairportnetwork en0` : get currently connected Wi-Fi network name (Wi-Fi device usually en0 or en1)
- `networksetup -setairportnetwork en0 ssid password` : connect to a particular Wi-Fi network

## Categories
network, hardware, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/networksetup
