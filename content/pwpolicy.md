# pwpolicy

Get and set password policies.

## Usage
- `sudo pwpolicy -a authenticator -u username -setpassword "new_password"` : change password for specified user
- `sudo pwpolicy -u username -disableuser` : disable a user account
- `sudo pwpolicy -u username -enableuser` : enable a user account
- `sudo pwpolicy -u username -gethashtypes` : get list of password hashes stored on disk for a user account

## Categories
users, macos

## Links
- More information: https://keith.github.io/xcode-man-pages/pwpolicy.8.html
- tldr page: https://tldr.inbrowser.app/pages/osx/pwpolicy
