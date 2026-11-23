# aa-status

List currently loaded AppArmor modules.

## Usage
- `sudo aa-status` : check status
- `sudo aa-status --json` : display status in JSON format
- `sudo aa-status --pretty-json` : display status in pretty JSON format
- `sudo aa-status --profiled` : display the number of loaded policies
- `sudo aa-status --enforced` : display the number of loaded enforcing policies
- `sudo aa-status --complaining` : display the number of loaded non-enforcing policies
- `sudo aa-status --kill` : display the number of loaded enforcing policies that kill tasks

## See also
- aa-complain
- aa-disable
- aa-enforce

## Categories
users, desktop, text

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-status.8
- tldr page: https://tldr.inbrowser.app/pages/linux/aa-status
