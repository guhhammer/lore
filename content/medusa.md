# medusa

A modular and parallel login brute-forcer for a variety of protocols.

## Usage
- `medusa -d` : list all installed modules
- `medusa -M ssh|http|web-form|postgres|ftp|mysql|... -q` : show usage example of a specific module (use `medusa -d` for listing all installed modules)
- `medusa -M ftp -h host -U path/to/username_file -P path/to/password_file` : execute brute force against an FTP server using a file containing usernames and a file containing passwords
- `medusa -M HTTP -h host -u username -p password -m USER-AGENT:"Agent"` : execute a login attempt against an HTTP server using the username, password, and user-agent specified
- `medusa -M mysql -h host -U path/to/username_file -p hash -m PASS:HASH` : execute a brute force against a MySQL server using a file containing usernames and a hash
- `medusa -M smbnt -H path/to/hosts_file -C path/to/pwdump_file -u username -m PASS:HASH` : execute a brute force against a list of SMB servers using a username and a pwdump file

## Categories
network, files, users

## Links
- More information: https://manned.org/medusa
- tldr page: https://tldr.inbrowser.app/pages/common/medusa
