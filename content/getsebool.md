# getsebool

Get SELinux boolean value.

## Usage
- `getsebool httpd_can_connect_ftp` : show the current setting of a boolean
- `getsebool -a` : show the current setting of all booleans
- `sudo semanage boolean -l` : show the current setting of all booleans with explanations

## See also
- semanage boolean
- setsebool

## Links
- More information: https://manned.org/getsebool
- tldr page: https://tldr.inbrowser.app/pages/linux/getsebool
