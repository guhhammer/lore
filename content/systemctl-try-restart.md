# systemctl try-restart

Restart one or more units only if they are currently running.

## Usage
- `systemctl try-restart unit` : restart a specific unit if it is running
- `systemctl try-restart unit1 unit2 ...` : restart multiple units if they are running
- `systemctl try-restart 'pattern'` : restart all units matching a pattern if they are running

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-try-restart
