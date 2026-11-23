# setfacl

Set file access control lists (ACL).

## Usage
- `setfacl -m u:username:rw path/to/file_or_directory` : modify ACL of a file for user with read and write access
- `setfacl -d -m u::rw path/to/file_or_directory` : modify default ACL of a file for all users
- `setfacl -x u:username path/to/file_or_directory` : remove ACL of a file for a user
- `setfacl -b path/to/file_or_directory` : remove all ACL entries of a file

## Categories
files, users

## Links
- More information: https://manned.org/setfacl
- tldr page: https://tldr.inbrowser.app/pages/linux/setfacl
