# systemctl hybrid-sleep

Put the system into hybrid sleep, which combines suspend-to-RAM and hibernate.

## Usage
- `systemctl hybrid-sleep` : put the system into hybrid sleep immediately
- `systemctl hybrid-sleep -f` : force hybrid sleep even if inhibitors are present
- `systemctl hybrid-sleep --no-wall` : put the system into hybrid sleep without sending a wall message to logged-in users

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/systemctl-hybrid-sleep
