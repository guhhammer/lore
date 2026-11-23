# semodule

Manage SELinux policy modules.

## Usage
- `sudo semodule -l` : list all installed policy modules
- `sudo semodule -i path/to/module.pp` : install a new policy module
- `sudo semodule -r module_name` : remove a policy module
- `sudo semodule -e module_name` : enable a policy module
- `sudo semodule -d module_name` : disable a policy module
- `sudo semodule -R` : reload all policy modules
- `sudo semodule -l -v` : display the version of installed policy modules

## See also
- audit2allow
- semanage

## Categories
users, development, system

## Links
- More information: https://manned.org/semodule
- tldr page: https://tldr.inbrowser.app/pages/linux/semodule
