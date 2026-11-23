# slocate

Secure variant of GNU Locate.

## Usage
- `slocate -q` : enable quiet mode to suppress error messages
- `slocate -n number` : limit the number of results shown
- `slocate -u` : build an `slocate` database starting at path `/`
- `slocate -U path/to/directory` : build an `slocate` database starting at a given directory
- `slocate -c` : update an `slocate` database using the default `/etc/updatedb.conf` configuration
- `slocate -l 0|1` : set the security level of `slocate`, with `0` being disabled, and `1` being secure
- `slocate -d path/to/directory` : specify the database that `slocate` should search in
- `slocate -r regex` : search the `slocate` database using a specific `regex` string

## See also
- locate

## Categories
files, databases, security

## Links
- More information: https://manned.org/slocate
- tldr page: https://tldr.inbrowser.app/pages/common/slocate
