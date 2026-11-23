# mount.cifs

Mount SMB (Server Message Block) or CIFS (Common Internet File System) shares. Note: You can also do the same thing by passing the `-t cifs` option to `mount`.

## Usage
- `mount.cifs -o user=username //server/share_name path/to/mount_point` : connect using the specified username or `$USER` by default (you will be prompted for a password)
- `mount.cifs -o guest //server/share_name path/to/mount_point` : connect as the guest user (without a password)
- `mount.cifs -o uid=user_id|username,gid=group_id|groupname //server/share_name path/to/mount_point` : set ownership information for the mounted directory

## Categories
disks, files, users

## Links
- More information: https://manned.org/mount.cifs
- tldr page: https://tldr.inbrowser.app/pages/linux/mount.cifs
