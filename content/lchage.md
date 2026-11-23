# lchage

Display or change user password policy.

## Usage
- `sudo lchage --date -1 username` : disable password expiration for the user
- `sudo lchage --list username` : display the password policy for the user
- `sudo lchage --maxdays number_of_days username` : require password change for the user a certain number of days after the last password change
- `sudo lchage --warndays number_of_days username` : start warning the user a certain number of days before the password expires

## Categories
users, desktop

## Links
- More information: https://manned.org/lchage
- tldr page: https://tldr.inbrowser.app/pages/linux/lchage
