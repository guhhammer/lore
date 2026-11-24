# passwd

Change a user's password.

## Usage
- `passwd` : change the password of the current user interactively
- `sudo passwd username` : change the password of a specific user
- `passwd -S` : get the current status of the user
- `passwd -d` : make the password of the account blank (it will set the named account passwordless)
- `yes password | passwd` : set password programmatically (ideal for install scripts)

## See also
- chpasswd

## Categories
users, security

## Links
- More information: https://manned.org/passwd
- tldr page: https://tldr.inbrowser.app/pages/common/passwd
- Wikipedia: https://en.wikipedia.org/wiki/Passwd
