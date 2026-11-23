# mount.smb3

This command is an alias of `mount.cifs`. Note: For SMB versions before 3 you have to use `mount.cifs` instead.

## Usage
- `mount.cifs -o user=username //server/share_name path/to/mount_point` : connect using the specified username or `$USER` by default (you will be prompted for a password)
- `mount.cifs -o guest //server/share_name path/to/mount_point` : connect as the guest user (without a password)
- `mount.cifs -o uid=user_id|username,gid=group_id|groupname //server/share_name path/to/mount_point` : set ownership information for the mounted directory

## Categories
disks, users, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/linux/mount.smb3
