# semanage fcontext

Manage persistent SELinux security context rules on files/directories.

## Usage
- `sudo semanage fcontext -l` : list all file labeling rules
- `sudo semanage fcontext -lCn` : list all user-defined file labeling rules without headings
- `sudo semanage fcontext -a -t samba_share_t '/mnt/share(/.*)?'` : add a user-defined rule that labels any path which matches a PCRE `regex`
- `sudo semanage fcontext -a -e /path/to/ref /path/to/target` : add a user-defined rule that creates a labeling equivalence between two subpaths
- `sudo semanage fcontext -d '/mnt/share(/.*)?'` : delete a user-defined rule using its PCRE `regex`
- `restorecon -Rv path/to/directory` : relabel a directory recursively by applying the new rules

## See also
- matchpathcon
- secon
- chcon
- restorecon

## Categories
files, users

## Links
- More information: https://manned.org/semanage-fcontext
- tldr page: https://tldr.inbrowser.app/pages/linux/semanage-fcontext
