# airdecap-ng

Decrypt a WEP, WPA, or WPA2 encrypted capture file. Part of Aircrack-ng network software suite.

## Usage
- `airdecap-ng -b ap_mac path/to/capture.cap` : remove wireless headers from an open network capture file and use the access point's MAC address to filter
- `airdecap-ng -w hex_key path/to/capture.cap` : decrypt a wEP encrypted capture file using the key in hex format
- `airdecap-ng -e extended_ssid -p password path/to/capture.cap` : decrypt a WPA/WPA2 encrypted capture file using the access point's essid and password
- `airdecap-ng -l -e extended_ssid -p password path/to/capture.cap` : decrypt a WPA/WPA2 encrypted capture file preserving the headers using the access point's essid and password
- `airdecap-ng -b ap_mac -e extended_ssid -p password path/to/capture.cap` : decrypt a WPA/WPA2 encrypted capture file using the access point's essid and password and use its MAC address to filter

## Categories
files, security, network

## Links
- More information: https://www.aircrack-ng.org/doku.php?id=airdecap-ng
- tldr page: https://tldr.inbrowser.app/pages/common/airdecap-ng
