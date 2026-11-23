# nxc

Network service enumeration and exploitation tool. Some subcommands such as `smb` have their own usage documentation.

## Usage
- `nxc smb|ssh|ldap|ftp|wmi|winrm|rdp|vnc|mssql -L` : list available modules for the specified protocol
- `nxc smb|ssh|ldap|ftp|wmi|winrm|rdp|vnc|mssql -M module_name --options` : list the options available for the specified module
- `nxc smb|ssh|ldap|ftp|wmi|winrm|rdp|vnc|mssql -M module_name -o OPTION_NAME=option_value` : specify an option for a module
- `nxc smb|ssh|ldap|ftp|wmi|winrm|rdp|vnc|mssql -h` : view the options available for the specified protocol

## Categories
network, system, documents

## Links
- More information: https://www.netexec.wiki/getting-started/selecting-and-using-a-protocol
- tldr page: https://tldr.inbrowser.app/pages/common/nxc
