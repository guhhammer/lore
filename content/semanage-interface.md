# semanage interface

Manage SELinux network interface type definitions.

## Usage
- `sudo semanage interface -l` : list all interface type definitions
- `sudo semanage interface -a -t type_name interface_name` : add a network interface type definition
- `sudo semanage interface -d interface_name` : delete a network interface type definition
- `sudo semanage interface -m -t type_name interface_name` : modify a network interface type definition
- `sudo semanage interface -l -C` : list interface type definitions in a customized format

## See also
- semanage port

## Categories
network, users

## Links
- More information: https://manned.org/semanage-interface
- tldr page: https://tldr.inbrowser.app/pages/linux/semanage-interface
