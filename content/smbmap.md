# smbmap

Enumerate samba share drives across an entire domain.

## Usage
- `smbmap --host-file path/to/file` : enumerate hosts with NULL sessions enabled and open shares
- `smbmap -u username --prompt -H ip_address` : display SMB shares and permissions on a Host, prompting for user's password or NTLM hash
- `smbmap -u username --prompt -H ip_address -x command` : execute a shell command on a remote system
- `smbmap --host-file path/to/file -u username -p password -q` : enumerate hosts and check SMB file permissions
- `smbmap -u username -p password -d domain -H ip_or_hostname` : connect to an ip or hostname through smb using a username and password
- `smbmap --host-file path/to/file -u username -p password -q -R --depth n --exclude sharename -A filepattern` : locate and download files Recursively up to `n` levels depth, searching for filename pattern (`regex`), and excluding certain shares
- `smbmap -u username -p password -d domain -H ip_or_hostname --upload path/to/file '/share_name/remote_filename'` : upload file through smb using username and password
- `smbmap -u username --prompt -H ip_address -R -F pattern` : display SMB shares and recursively list directories and files, searching for file content matching a `regex`

## Categories
files, network, users

## Links
- More information: https://github.com/ShawnDEvans/smbmap#help
- tldr page: https://tldr.inbrowser.app/pages/common/smbmap
