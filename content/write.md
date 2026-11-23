# write

Write a message on the terminal of a specified logged in user (`<Ctrl c>` to stop writing messages). Use the `who` command to find out all terminal_ids of all active users active on the system.

## Usage
- `write username terminal_id` : send a message to a given user on a given terminal ID
- `write testuser tty5` : send message to "testuser" on terminal `/dev/tty5`
- `write johndoe pts/5` : send message to "johndoe" on pseudo terminal `/dev/pts/5`

## See also
- mesg

## Categories
users, shell

## Links
- More information: https://manned.org/write.1p
- tldr page: https://tldr.inbrowser.app/pages/common/write
