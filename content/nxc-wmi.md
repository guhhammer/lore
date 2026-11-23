# nxc wmi

Pentest and exploit the Windows Management Instrumentation (WMI).

## Usage
- `nxc wmi 192.168.178.2 -u path/to/usernames.txt -p path/to/passwords.txt` : search for valid credentials by trying out every combination in the specified lists of usernames and passwords
- `nxc wmi 192.168.178.2 -u username -p password --local-auth` : authenticate via local authentication (as opposed to authenticating to the domain)
- `nxc wmi 192.168.178.2 -u username -p password --wmi wmi_query` : issue the specified WMI query
- `nxc wmi 192.168.178.2 -u username -p password -x command` : execute the specified command on the targeted host

## Categories
users, network

## Links
- More information: https://www.netexec.wiki/wmi-protocol/password-spraying
- tldr page: https://tldr.inbrowser.app/pages/common/nxc-wmi
