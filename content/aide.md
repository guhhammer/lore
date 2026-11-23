# aide

Advanced Intrusion Detection Environment to validate file integrity.

## Usage
- `sudo aide -i` : initialize the database
- `sudo aide -C` : check the database for inconsistencies
- `sudo aide -E` : compare two databases according to definitions in the config file
- `sudo aide -u` : check and update the database non-interactively
- `sudo aide -c path/to/config_file` : define a config file to override the default `aide.conf`
- `sudo aide -l regex` : use `regex` to limit AIDE to a specific string
- `sudo aide -r reporterurl` : send reporter results to a URL

## Categories
users, files, databases

## Links
- More information: https://manned.org/aide
- tldr page: https://tldr.inbrowser.app/pages/linux/aide
