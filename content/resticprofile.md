# resticprofile

Configuration profiles manager for restic backup.

## Usage
- `resticprofile` : list all saved snapshots
- `resticprofile backup` : run a backup on the default profile
- `resticprofile -n "profile_name" backup` : run a backup with a specific profile (default profile is "default")
- `resticprofile --dry-run backup` : simulate a backup and show the underlying restic commands
- `resticprofile profiles` : display the names of all profiles from the configuration file
- `resticprofile generate --bash-completion|--zsh-completion` : generate shell completions
- `resticprofile show -n "profile_name"` : show all details of a profile

## See also
- restic

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/resticprofile
