# chage

Change user account and password expiry information.

## Usage
- `chage -l username` : list password information for the user
- `sudo chage -M 10 username` : enable password expiration in 10 days
- `sudo chage -M -1 username` : disable password expiration
- `sudo chage -E YYYY-MM-DD username` : set account expiration date
- `sudo chage -d 0 username` : force user to change password on next log in
- `sudo chage -E -1 username` : re-enable an account

## Categories
users

## Links
- More information: https://manned.org/chage
- tldr page: https://tldr.inbrowser.app/pages/linux/chage
