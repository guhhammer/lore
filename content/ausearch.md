# ausearch

Query the Linux audit log for events. Part of the `audit` package.

## Usage
- `sudo ausearch -m avc` : search for all SELinux AVC denial events
- `sudo ausearch -c httpd` : search for events related to a specific executable
- `sudo ausearch -ui 1000` : search for events from a specific user
- `sudo ausearch -ts recent` : search for events in the last 10 minutes
- `sudo ausearch -m user_login -sv no` : search for failed login attempts
- `sudo ausearch -f path/to/file` : search for events related to a specific file
- `sudo ausearch -m avc --raw` : display results in raw format for further processing

## See also
- audit2why
- audit2allow
- aureport

## Categories
users, databases, files

## Links
- More information: https://manned.org/ausearch
- tldr page: https://tldr.inbrowser.app/pages/linux/ausearch
