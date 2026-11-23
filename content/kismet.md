# kismet

A wireless network and device detector, sniffer, wardriving tool, and WIDS (wireless intrusion detection) framework.

## Usage
- `sudo kismet -c wlan0` : capture packets from a specific wireless interface
- `sudo kismet -c wlan0,wlan1 -m` : monitor multiple channels on a wireless interface
- `sudo kismet -c wlan0 -d path/to/output` : capture packets and save them to a specific directory
- `sudo kismet -c wlan0 -f path/to/config.conf` : start Kismet with a specific configuration file
- `sudo kismet -c wlan0 --log-to-db` : monitor and log data to an SQLite database
- `sudo kismet -c wlan0 --data-source=rtl433` : monitor using a specific data source
- `sudo kismet -c wlan0 --enable-alert=new_ap` : enable alerts for specific events
- `sudo kismet -c wlan0 --info basic_ssid` : display detailed information about a specific AP's packets

## Categories
users, network, monitoring

## Links
- More information: https://www.kismetwireless.net/docs/readme/starting/commandline/
- tldr page: https://tldr.inbrowser.app/pages/common/kismet
