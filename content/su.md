# su

Switch shell to another user.

## Usage
- `su` : switch to superuser (requires the root password)
- `su username` : switch to a given user (requires the user's password)
- `su - username` : switch to a given user and simulate a full login shell
- `su - username -c "command"` : execute a command as another user
- `su -s /path/to/shell username` : switch to a given user and use a specific shell (e.g., Zsh, fish, Bash)
- `su -h` : display help
- `su -V` : display version

## Categories
users

## Links
- More information: https://manned.org/su
- tldr page: https://tldr.inbrowser.app/pages/linux/su
- Wikipedia: https://en.wikipedia.org/wiki/Su_(Unix)
