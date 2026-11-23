# nxc winrm

Pentest and exploit Windows Remote Management (winrm).

## Usage
- `nxc winrm 192.168.178.2 -u path/to/usernames.txt -p path/to/passwords.txt` : search for valid credentials by trying out every combination in the specified lists of usernames and passwords
- `nxc winrm 192.168.178.2 -u username -p password -d domain_name` : specify the domain to authenticate to (avoids an initial SMB connection)
- `nxc winrm 192.168.178.2 -u username -p password -x whoami` : execute the specified command on the host
- `nxc winrm 192.168.178.2 -u username -p password --laps -X whoami` : execute the specified PowerShell command on the host as administrator using LAPS

## Categories
network, users

## Links
- More information: https://www.netexec.wiki/winrm-protocol/password-spraying
- tldr page: https://tldr.inbrowser.app/pages/common/nxc-winrm
