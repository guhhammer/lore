# init

Linux run level manager. Requires the SYSVINIT compile-time option to be enabled if using systemd.

## Usage
- `sudo init 5` : set the system to run a graphical environment
- `sudo init 3` : set the system to run multiuser terminal
- `init 0` : shut down the system
- `init 6` : reboot the system
- `sudo init 1` : set the system to run on terminal with only root user allowed and no networking

## Categories
system, users, development

## Links
- More information: https://manned.org/init.8
- tldr page: https://tldr.inbrowser.app/pages/linux/init
