# unshadow

Utility provided by the John the Ripper project to obtain the traditional Unix password file if the system uses shadow passwords.

## Usage
- `sudo unshadow /etc/passwd /etc/shadow` : combine the `/etc/shadow` and `/etc/passwd` of the current system
- `sudo unshadow path/to/passwd path/to/shadow` : combine two arbitrary shadow and password files

## Categories
users, files, development

## Links
- More information: https://www.openwall.com/john/doc/
- tldr page: https://tldr.inbrowser.app/pages/linux/unshadow
