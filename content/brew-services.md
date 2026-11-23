# brew services

Manage background services with `launchctl` on macOS or `systemctl` on Linux.

## Usage
- `brew services` : list all managed services for the current user
- `brew services info --all` : list more information about all managed services
- `brew services start formula` : start a service immediately and register it to launch at login (or boot)
- `brew services stop formula` : stop the service immediately and unregister it from launching at login (or boot)
- `brew services restart formula` : stop (if necessary) and start the service immediately and register it to launch at login (or boot)
- `brew services cleanup` : remove all unused services

## Categories
system, users

## Links
- More information: https://docs.brew.sh/Manpage#services-subcommand
- tldr page: https://tldr.inbrowser.app/pages/common/brew-services
