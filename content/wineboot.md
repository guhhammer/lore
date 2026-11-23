# wineboot

Simulate a Windows reboot, or initialize, update, or shut down a Wine prefix.

## Usage
- `wineboot` : perform a simulated reboot of the Wine prefix
- `wineboot -u` : initialize or update the prefix (e.g. after upgrading Wine)
- `wineboot -r` : restart Wine without performing a full boot
- `wineboot -s` : shut down the Wine session without rebooting
- `wineboot -ef` : end the current session, forcing unresponsive processes to close
- `wineboot -k` : kill all processes in the prefix without any cleanup
- `wineboot -h` : display help

## Categories
system

## Links
- More information: https://manned.org/wineboot
- tldr page: https://tldr.inbrowser.app/pages/linux/wineboot
