# netsh wlan

Manage wireless networks.

## Usage
- `netsh wlan show networks` : show all available wireless networks
- `netsh wlan connect name=ssid` : connect to a wireless network with a specific SSID
- `netsh wlan disconnect` : disconnect from the current wireless network
- `netsh wlan show interfaces` : show current wireless network interfaces and status
- `netsh wlan export profile name=ssid folder=C:\path\to\folder key=clear` : export a wireless network profile to an XML file
- `netsh wlan delete profile name=ssid` : delete a saved wireless network profile
- `netsh wlan set hostednetwork mode=allow ssid=ssid key=password` : enable hosted network (turn PC into Wi-Fi hotspot)
- `netsh wlan start hostednetwork` : start the hosted network

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
network, files, windows

## Links
- More information: https://www.serverwatch.com/guides/netsh-commands/
- tldr page: https://tldr.inbrowser.app/pages/windows/netsh-wlan
