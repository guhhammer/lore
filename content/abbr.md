# abbr

Manage abbreviations for the fish shell. User-defined words are replaced with longer phrases after they are entered.

## Usage
- `abbr -a abbreviation_name command command_arguments` : add a new abbreviation
- `abbr --rename old_name new_name` : rename an existing abbreviation
- `abbr -e abbreviation_name` : erase an existing abbreviation
- `ssh host_name abbr -s | source` : import the abbreviations defined on another host over SSH

## Categories
network, shell, users

## Links
- More information: https://fishshell.com/docs/current/cmds/abbr.html
- tldr page: https://tldr.inbrowser.app/pages/linux/abbr
