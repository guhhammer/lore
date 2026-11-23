# timedatectl

Control the system time and date.

## Usage
- `timedatectl` : check the current system clock time
- `timedatectl set-time "yyyy-MM-dd hh:mm:ss"` : set the local time of the system clock directly
- `timedatectl list-timezones` : list available timezones
- `timedatectl set-timezone timezone` : set the system timezone
- `timedatectl set-ntp on` : enable Network Time Protocol (NTP) synchronization
- `timedatectl timesync-status` : show information about time synchronization (only available when using `systemd-timesyncd`)
- `timedatectl set-local-rtc 1` : change the hardware clock time standard to localtime

## Categories
system, network

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/timedatectl.html
- tldr page: https://tldr.inbrowser.app/pages/linux/timedatectl
