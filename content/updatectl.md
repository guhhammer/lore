# updatectl

System update utility.

## Usage
- `updatectl check` : check to see if the system has any available updates
- `updatectl update` : update to the latest version
- `updatectl` : show update targets
- `updatectl list target` : show data about a target and its versions
- `updatectl --offline list target` : return local data about a target without fetching from the network
- `updatectl --reboot update target` : apply an update to a target and then reboot the system
- `updatectl vacuum target` : clean up old versions of a specified target
- `updatectl -h` : display help

## Categories
network, system

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/updatectl.html
- tldr page: https://tldr.inbrowser.app/pages/linux/updatectl
