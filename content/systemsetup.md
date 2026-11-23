# systemsetup

Configure System Preferences machine settings.

## Usage
- `systemsetup -setremotelogin on` : enable remote login (SSH)
- `systemsetup -settimezone "US/Pacific" -setnetworktimeserver us.pool.ntp.org -setusingnetworktime on` : specify timezone, NTP Server, and enable network time
- `systemsetup -setsleep off -setrestartpowerfailure on -setrestartfreeze on` : make the machine never sleep and automatically restart on power failure or kernel panic
- `systemsetup -liststartupdisks` : list valid startup disks
- `systemsetup -setstartupdisk path/to/directory` : specify a new startup disk

## Categories
network, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/systemsetup
