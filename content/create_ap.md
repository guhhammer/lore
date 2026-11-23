# create_ap

Create an AP (Access Point) at any channel.

## Usage
- `create_ap wlan0 eth0 access_point_ssid` : create an open network with no passphrase
- `create_ap wlan0 eth0 access_point_ssid passphrase` : use a WPA + WPA2 passphrase
- `create_ap -n wlan0 access_point_ssid passphrase` : create an access point without Internet sharing
- `create_ap -m bridge wlan0 eth0 access_point_ssid passphrase` : create a bridged network with Internet sharing
- `create_ap -m bridge wlan0 br0 access_point_ssid passphrase` : create a bridged network with Internet sharing and a pre-configured bridge interface
- `create_ap wlan0 wlan0 access_point_ssid passphrase` : create an access port for Internet sharing from the same Wi-Fi interface
- `create_ap --driver wifi_adapter wlan0 eth0 access_point_ssid passphrase` : choose a different Wi-Fi adapter driver

## Categories
network

## Links
- More information: https://github.com/oblique/create_ap
- tldr page: https://tldr.inbrowser.app/pages/linux/create_ap
