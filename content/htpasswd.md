# htpasswd

Create and manage htpasswd files to protect web server directories using basic authentication.

## Usage
- `htpasswd -c path/to/file username` : create/overwrite htpasswd file
- `htpasswd path/to/file username` : add user to htpasswd file or update existing user
- `htpasswd -b path/to/file username password` : add user to htpasswd file in batch mode without an interactive password prompt (for script usage)
- `htpasswd -D path/to/file username` : delete user from htpasswd file
- `htpasswd -v path/to/file username` : verify user password
- `htpasswd -nbm username password` : display a string with username (plain text) and password (MD5)

## Categories
files, users

## Links
- More information: https://httpd.apache.org/docs/current/programs/htpasswd.html
- tldr page: https://tldr.inbrowser.app/pages/common/htpasswd
