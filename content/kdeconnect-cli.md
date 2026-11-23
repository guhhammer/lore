# kdeconnect-cli

Use KDE Connect for sharing files or text to a device, ringing it, unlocking it, and much more.

## Usage
- `kdeconnect-cli -l` : list all devices
- `kdeconnect-cli --list-available` : list available (paired and reachable) devices
- `kdeconnect-cli --pair --device device_id` : request pairing with a specific device, specifying its ID
- `kdeconnect-cli --ring --name "device_name"` : ring a device, specifying its name
- `kdeconnect-cli --share url|path/to/file --device device_id` : share a URL or file with a paired device, specifying its ID
- `kdeconnect-cli --name "device_name" --send-sms "message" --destination phone_number --attachment path/to/file` : send an SMS with an optional attachment to a specific number
- `kdeconnect-cli --name "device_name" --unlock` : unlock a specific device
- `kdeconnect-cli --name "device_name" --send-keys key` : simulate a key press on a specific device

## Categories
hardware, files, network

## Links
- More information: https://manned.org/kdeconnect-cli
- tldr page: https://tldr.inbrowser.app/pages/common/kdeconnect-cli
