# nmcli device

Manage network interfaces and establish new Wi-Fi connections using NetworkManager.

## Usage
- `nmcli d` : print the statuses of all network interfaces
- `nmcli d w` : print the available Wi-Fi access points
- `nmcli d w c ssid -a` : connect to a Wi-Fi network with the specified SSID (you will be prompted for a password)
- `nmcli d w ho ifname wlan0 ssid "hotspot_ssid" password "password"` : create a Wi-Fi hotspot
- `nmcli d w s` : print the password and QR code for the current Wi-Fi network
- `nmcli d sh wlan0` : print detailed information about a device

## Categories
network, hardware, documents

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/nmcli-device
