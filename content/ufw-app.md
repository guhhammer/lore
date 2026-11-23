# ufw app

Manage `ufw` application profiles and their associated firewall rules. Application profiles are defined in `/etc/ufw/applications.d/`.

## Usage
- `sudo ufw app list` : list available application profiles
- `sudo ufw app info profile_name|all` : display information about a specific or all profiles
- `sudo ufw allow|deny "profile_name"` : allow or deny traffic using a specific profile
- `sudo ufw app update profile_name|all` : update firewall rules for a modified profile or all profiles
- `sudo ufw app update --add-new "profile_name"` : update a profile, adding new rules if none exist
- `sudo ufw app default allow|deny|reject|skip` : configure the default behavior of `update --add-new`

## Categories
users, security

## Links
- More information: https://manned.org/ufw#head8
- tldr page: https://tldr.inbrowser.app/pages/linux/ufw-app
