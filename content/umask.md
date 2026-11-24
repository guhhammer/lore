# umask

Manage the read/write/execute permissions that are masked out (i.e. restricted) for newly created files by the user.

## Usage
- `umask` : display the current mask in octal notation
- `umask -S` : display the current mask in symbolic (human-readable) mode
- `umask a+r` : change the mask symbolically to allow read permission for all users (the rest of the mask bits are unchanged)
- `umask 077` : set the mask (using octal) to restrict no permissions for the file's owner, and restrict all permissions for everyone else

## Categories
users, shell

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-umask
- tldr page: https://tldr.inbrowser.app/pages/common/umask
- Wikipedia: https://en.wikipedia.org/wiki/Umask
