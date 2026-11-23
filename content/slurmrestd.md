# slurmrestd

Interface to Slurm via REST API. Note: Can be used in two modes: Inetd Mode and Listen Mode.

## Usage
- `slurmrestd -g group_id host:port|unix:/path/to/socket` : change the group ID (and drop supplemental groups) before processing client requests
- `slurmrestd -a authentication_plugins host:port|unix:/path/to/socket` : specify a comma-delimited list of authentication plugins to load
- `slurmrestd -f path/to/file` : read Slurm configuration from the specified file
- `slurmrestd -u user_id` : change user ID before processing client request
- `slurmrestd -h` : display help
- `slurmrestd -V` : display Version

## Categories
network, files, users

## Links
- More information: https://slurm.schedmd.com/slurmrestd.html
- tldr page: https://tldr.inbrowser.app/pages/linux/slurmrestd
