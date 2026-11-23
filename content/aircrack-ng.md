# aircrack-ng

Crack WEP and WPA/WPA2 keys from handshake in captured packets. Part of Aircrack-ng network software suite.

## Usage
- `aircrack-ng -w path/to/wordlist.txt path/to/capture.cap` : crack key from capture file using wordlist
- `aircrack-ng -p number -w path/to/wordlist.txt path/to/capture.cap` : crack key using multiple CPU threads from capture file using wordlist
- `aircrack-ng -w path/to/wordlist.txt -e extended_ssid path/to/capture.cap` : crack key from capture file using wordlist and the access point's essid
- `aircrack-ng -w path/to/wordlist.txt --bssid mac path/to/capture.cap` : crack key from capture file using wordlist and the access point's MAC address

## Categories
files, network

## Links
- More information: https://www.aircrack-ng.org/doku.php?id=aircrack-ng
- tldr page: https://tldr.inbrowser.app/pages/common/aircrack-ng
