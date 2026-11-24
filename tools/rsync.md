# rsync

rsync copies files and folders, locally or over ssh, sending only what changed. It is the tool for backups and for moving big folders.

## Tasks
### Copy
- `rsync -av SRC/ DEST/` : copy a folder, keeping permissions and times
- `rsync -avh --progress SRC/ DEST/` : show progress and readable sizes
- `rsync -av FILE DEST/` : copy one file
- `rsync -avn SRC/ DEST/` : a dry run: show what would happen, copy nothing
- `rsync -avz SRC/ DEST/` : compress while copying (useful over slow links)
### Over ssh
- `rsync -avz SRC/ USER@HOST:/path/` : send a folder to a server
- `rsync -avz USER@HOST:/path/ DEST/` : fetch a folder from a server
- `rsync -avz -e "ssh -p PORT" SRC/ USER@HOST:/path/` : use another ssh port
- `rsync -avz -e "ssh -i ~/.ssh/KEY" SRC/ USER@HOST:/path/` : use a chosen key
### Mirror and back up
- `rsync -av --delete SRC/ DEST/` : make DEST identical to SRC (deletes extra files in DEST)
- `rsync -avn --delete SRC/ DEST/` : preview what a mirror would delete
- `rsync -av --backup --backup-dir=../old SRC/ DEST/` : keep replaced files in another folder
- `rsync -a --link-dest=../last SRC/ DEST/today/` : snapshot backups that share unchanged files
### Choose what to copy
- `rsync -av --exclude='node_modules' SRC/ DEST/` : skip a name
- `rsync -av --exclude-from=FILE SRC/ DEST/` : skip every pattern listed in a file
- `rsync -av --include='*.jpg' --exclude='*' SRC/ DEST/` : only some files
- `rsync -av --max-size=10m SRC/ DEST/` : skip big files
- `rsync -av --files-from=LIST SRC/ DEST/` : copy only the paths in a list
### Resume and limit
- `rsync -avP SRC/ DEST/` : show progress and keep partial files so you can resume
- `rsync --bwlimit=1000 -av SRC/ DEST/` : limit speed to about 1000 KB/s
- `rsync -av --remove-source-files SRC/ DEST/` : move: delete files from SRC after copying
- `rsync -a --info=progress2 SRC/ DEST/` : one overall progress line

## Notes
- A trailing slash on the source matters: `SRC/` copies what is inside, `SRC` copies the folder itself.
- `--delete` removes files: run it with `-n` first.
- rsync must be installed on both machines for remote copies.
- `-a` already includes recursion, links, permissions, times, owner and group.

## See also
- scp
- ssh-keys
- tar-and-zip
- cp

## Links
- Manual: https://download.samba.org/pub/rsync/rsync.1
- Project: https://rsync.samba.org

## Categories
files network
