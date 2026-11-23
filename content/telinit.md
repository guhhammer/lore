# telinit

Change SysV runlevel. Since the concept SysV runlevels is obsolete the runlevel requests will be transparently translated into systemd unit activation requests.

## Usage
- `telinit 0` : power off the machine
- `telinit 6` : reboot the machine
- `telinit 2..5` : change SysV run level
- `telinit 1` : change to rescue mode
- `telinit q` : reload daemon configuration
- `telinit --no-wall value` : do not send a wall message before reboot/power-off (6/0)

## Categories
system

## Links
- More information: https://manned.org/telinit
- tldr page: https://tldr.inbrowser.app/pages/linux/telinit
