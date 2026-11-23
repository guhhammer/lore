# gnuchcon

This command is an alias of GNU `chcon`.

## Usage
- `ls -lZ path/to/file` : view security context of a file
- `chcon --reference reference_file target_file` : change the security context of a target file, using a reference file
- `chcon user:role:type:range/level filename` : change the full SELinux security context of a file
- `chcon -u user filename` : change only the user part of SELinux security context
- `chcon -r role filename` : change only the role part of SELinux security context
- `chcon -t type filename` : change only the type part of SELinux security context
- `chcon -l range/level filename` : change only the range/level part of SELinux security context

## Categories
files, shell, users

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/gnuchcon
