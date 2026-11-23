# shutdown

Shut down and reboot the system.

## Usage
- `shutdown -h now` : power off (halt) immediately
- `shutdown -r now` : reboot immediately
- `shutdown -r +5 &` : reboot in 5 minutes
- `shutdown -h 13:00` : shut down at 1:00 pm (Uses 24h clock)
- `shutdown -c` : cancel a pending shutdown/reboot operation

## See also
- poweroff

## Categories
danger, system

## Links
- More information: https://manned.org/shutdown.8
- tldr page: https://tldr.inbrowser.app/pages/linux/shutdown
