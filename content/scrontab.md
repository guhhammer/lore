# scrontab

Manage Slurm crontab files.

## Usage
- `scrontab path/to/file` : install a new crontab from the specified file
- `scrontab -e` : edit the crontab of the current user
- `scrontab --user=user_id -e` : edit the crontab of the specified user
- `scrontab -r` : remove the current crontab
- `scrontab -l` : print the crontab of the current user to `stdout`

## Categories
users, files

## Links
- More information: https://slurm.schedmd.com/scrontab.html
- tldr page: https://tldr.inbrowser.app/pages/linux/scrontab
