# apparmor_status

This command is an alias of `aa-status`.

## Usage
- `sudo aa-status` : check status
- `sudo aa-status --json` : display status in JSON format
- `sudo aa-status --pretty-json` : display status in pretty JSON format
- `sudo aa-status --profiled` : display the number of loaded policies
- `sudo aa-status --enforced` : display the number of loaded enforcing policies
- `sudo aa-status --complaining` : display the number of loaded non-enforcing policies
- `sudo aa-status --kill` : display the number of loaded enforcing policies that kill tasks

## Categories
users, desktop, text

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/apparmor_status
