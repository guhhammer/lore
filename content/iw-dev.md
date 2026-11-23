# iw dev

Show and manipulate wireless devices. For a list of channels, frequencies, and reg information: <https://wireless.docs.kernel.org/en/latest/en/developers/documentation/channellist.html>.

## Usage
- `sudo iw dev wlanX set type monitor` : set device to monitor mode (interface must be down first. See also: `ip link`)
- `sudo iw dev wlanX set type managed` : set device to managed mode (interface must be down first)
- `sudo iw dev wlanX set channel channel_number` : set device Wi-Fi channel (device must first be in monitor mode with the interface up)
- `sudo iw dev wlanX set freq freq_in_mhz` : set device Wi-Fi frequency in Mhz (device must first be in monitor mode with the interface up)
- `iw dev wlanX station dump` : show all known station info
- `sudo iw dev wlanX interface add "vif_name" type monitor addr 12:34:56:aa:bb:cc` : create a virtual interface in monitor mode with a specific MAC address
- `sudo iw dev "vif_name" del` : delete virtual interface

## Categories
network, hardware, users

## Links
- More information: https://wireless.docs.kernel.org/en/latest/en/users/documentation/iw.html
- tldr page: https://tldr.inbrowser.app/pages/linux/iw-dev
