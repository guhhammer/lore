# physlock

Lock all consoles and virtual terminals.

## Usage
- `physlock` : lock every console (require current user or root to unlock)
- `physlock -m` : mute kernel messages on console while locked
- `physlock -s` : disable SysRq mechanism while locked
- `physlock -p "Locked!"` : display a message before the password prompt
- `physlock -d` : fork and detach physlock (useful for suspend or hibernate scripts)

## Links
- More information: https://github.com/xyb3rt/physlock#usage
- tldr page: https://tldr.inbrowser.app/pages/linux/physlock
