# sport

Search and install SlackBuilds.

## Usage
- `sudo mkdir -p /usr/ports && sudo rsync -av rsync://slackbuilds.org /slackbuilds/$(awk '{print $2}' /etc/slackware-version)/ /usr/ports/` : pull the list of SlackBuilds to run `sport` for the first time
- `sudo sport rsync` : pull in any updates to the system's tree via `rsync`
- `sport search "keyword"` : search for a package by name
- `sport check package` : check if a package is installed
- `sport cat package` : display README and `.info` files of a package
- `sudo sport install package` : install a package once the dependencies are resolved
- `sudo sport install $(<path/to/list)` : install a list of packages from a file (format: packages separated by spaces)

## Categories
packages, users

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/sport
