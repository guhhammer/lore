# chntpw

A utility that can edit windows registry, reset user password, promote users to administrator by modifying the Windows SAM. Boot target machine with live cd like Kali Linux and run with elevated privileges.

## Usage
- `chntpw -l path/to/sam_file` : list all users in the SAM file
- `chntpw -u username path/to/sam_file` : edit user interactively
- `chntpw -i path/to/sam_file` : use chntpw interactively

## Categories
users, files, system

## Links
- More information: https://pogostick.net/~pnh/ntpasswd/MANUAL.txt
- tldr page: https://tldr.inbrowser.app/pages/linux/chntpw
- Wikipedia: https://en.wikipedia.org/wiki/Chntpw
