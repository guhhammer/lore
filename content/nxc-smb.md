# nxc smb

Pentest and exploit SMB servers.

## Usage
- `nxc smb 192.168.178.2 -u path/to/usernames.txt -p path/to/passwords.txt` : search for valid domain credentials by trying out every combination in the specified lists of usernames and passwords
- `nxc smb 192.168.178.2 -u path/to/usernames.txt -p path/to/passwords.txt --local-auth` : search for valid credentials for local accounts instead of domain accounts
- `nxc smb 192.168.178.0/24 -u username -p password --shares` : enumerate SMB shares and the specified users' access rights to them on the target hosts
- `nxc smb 192.168.178.30-45 -u username -H NTLM_hash --interfaces` : enumerate network interfaces on the target hosts, performing authentication via pass-the-hash
- `nxc smb path/to/target_list.txt -u '' -p '' -M zerologon -M petitpotam` : scan the target hosts for common vulnerabilities
- `nxc smb 192.168.178.2 -u username -p password -x command` : attempt to execute a command on the target hosts

## Categories
files, network, users

## Links
- More information: https://www.netexec.wiki/smb-protocol/generate-hosts-file
- tldr page: https://tldr.inbrowser.app/pages/common/nxc-smb
