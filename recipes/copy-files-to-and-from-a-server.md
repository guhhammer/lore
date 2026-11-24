# copy files to and from a server

Copy a file or a whole folder between this machine and another one over ssh.

## Steps
- `scp file.txt NAME@host:/path/` : send a file to the server
- `scp NAME@host:/path/file.txt .` : fetch a file from the server into this folder
- `scp -r NAME@host:/path/folder .` : fetch a whole folder
- `scp -P 2222 file.txt NAME@host:/path/` : when the server's ssh listens on another port
- `rsync -avz --progress folder/ NAME@host:/path/folder/` : send a folder, only what changed, with progress; it can be run again to resume
- `sftp NAME@host` : browse the server and copy with get and put

## Notes
- scp and rsync use the same login as ssh: keys, passwords and ~/.ssh/config hosts all work.

## See also
- scp
- rsync
- sftp
- ssh

## Categories
network files
