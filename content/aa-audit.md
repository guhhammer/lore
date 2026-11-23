# aa-audit

Set AppArmor security profiles to audit mode.

## Usage
- `sudo aa-audit profile_name` : set a profile to audit mode
- `sudo aa-audit profile1 profile2 ...` : set multiple profiles to audit mode
- `sudo aa-audit -d /path/to/profiles profile_name` : set a profile to audit mode from a specific directory
- `sudo aa-audit --force profile_name` : force audit mode even if already applied
- `sudo aa-audit --no-reload profile_name` : set a profile to audit mode without reloading it
- `sudo aa-audit -r profile_name` : remove audit mode for a profile
- `aa-audit -h` : display help

## Categories
users

## Links
- More information: https://gitlab.com/apparmor/apparmor/-/wikis/manpage_aa-audit.8
- tldr page: https://tldr.inbrowser.app/pages/linux/aa-audit
