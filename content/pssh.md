# pssh

Parallel SSH program.

## Usage
- `pssh -i -H "host1 host2" hostname --ip-addresses` : run a command on two hosts, and print its output on each server inline
- `pssh -H host1 -H host2 -o path/to/output_directory hostname --ip-addresses` : run a command and save the output to separate files
- `pssh -i -h path/to/hosts_file hostname --ip-addresses` : run a command on multiple hosts, specified in a new-line separated file
- `pssh -i -h path/to/hosts_file -A -l root_username hostname --ip-addresses` : run a command as root (this asks for the root password)
- `pssh -i -h path/to/hosts_file -x "-O VisualHostKey=yes" hostname --ip-addresses` : run a command with extra SSH arguments
- `pssh -i -h path/to/hosts_file -p 10 'cd dir; ./script.sh; exit'` : run a command limiting the number of parallel connections to 10

## Categories
files, network

## Links
- More information: https://manned.org/pssh
- tldr page: https://tldr.inbrowser.app/pages/common/pssh
