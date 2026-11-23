# solo

Interact with Solo hardware security keys.

## Usage
- `solo ls` : list connected Solos
- `solo key update` : update the currently connected Solo's firmware to the latest version
- `solo key wink --serial serial_number` : blink the LED of a specific Solo
- `solo key rng raw` : generate random bytes using the currently connected Solo's secure random number generator
- `solo monitor path/to/serial_port` : monitor the serial output of a Solo

## Links
- More information: https://github.com/solokeys/solo1-cli
- tldr page: https://tldr.inbrowser.app/pages/common/solo
