# pyrit

WPA/WPA2 cracking tool using computational power.

## Usage
- `pyrit benchmark` : display system cracking speed
- `pyrit list_cores` : list available cores
- `pyrit -e "extended_ssid" create_essid` : set eSSID
- `pyrit -r path/to/file.cap|pcap analyze` : read and analyze a specific packet capture file
- `pyrit -i path/to/file import_unique_passwords|unique_passwords|import_passwords` : read and import passwords to the current database
- `pyrit -o path/to/file export_passwords` : export passwords from database to a specific file
- `pyrit batch` : translate passwords with Pired Master Keys
- `pyrit -r path/to/file attack_db` : read the capture file and crack the password

## Categories
files, network

## Links
- More information: https://manned.org/pyrit
- tldr page: https://tldr.inbrowser.app/pages/linux/pyrit
