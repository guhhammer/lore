# dzdo

Execute privileged commands as root or another user via Active Directory roles. Similar to `sudo` but integrated with Delinea; supports Ansible become plugin.

## Usage
- `dzdo command` : run a command with elevated privileges
- `dzdo -u user command` : run a command as another user
- `dzdo -e path/to/file` : edit a file with elevated privileges using the default editor
- `dzdo -i` : launch an interactive login shell with elevated privileges
- `dzdo -s` : launch the default shell with elevated privileges
- `dzdo -l` : list allowed commands for the current user
- `dzdo -v` : validate and update authentication timestamp
- `dzdo -V` : display version

## Categories
users, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/dzdo
