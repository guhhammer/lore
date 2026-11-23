# semanage permissive

Manage persistent SELinux permissive domains. Note that this effectively makes the process unconfined. For long-term use, it is recommended to configure SELinux properly.

## Usage
- `sudo semanage permissive -l` : list all process types (a.k.a domains) that are in permissive mode
- `sudo semanage permissive -a httpd_t` : set permissive mode for a domain
- `sudo semanage permissive -d httpd_t` : unset permissive mode for a domain

## See also
- getenforce
- setenforce

## Categories
processes, users

## Links
- More information: https://manned.org/semanage-permissive
- tldr page: https://tldr.inbrowser.app/pages/linux/semanage-permissive
