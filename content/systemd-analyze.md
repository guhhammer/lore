# systemd-analyze

Analyze and debug system manager. Show timing details about the boot process of units (services, mount points, devices, sockets).

## Usage
- `systemd-analyze` : print the last system startup time
- `systemd-analyze blame` : list all running units, ordered by the time they took to initialize
- `systemd-analyze critical-chain` : print a tree of the time-critical chain of bootup units
- `systemd-analyze plot >path/to/file.svg` : create an SVG file showing when each system service started, highlighting the time that they spent on initialization
- `systemd-analyze dot | dot -T svg >path/to/file.svg` : plot a dependency graph and convert it to an SVG file
- `systemd-analyze security` : show security scores of running units
- `systemd-analyze cat-config udev/rules.d --tldr` : view all udev rules together without showing comments

## Categories
system, files, development

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-analyze.html
- tldr page: https://tldr.inbrowser.app/pages/linux/systemd-analyze
