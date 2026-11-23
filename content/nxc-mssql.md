# nxc mssql

Pentest and exploit Microsoft SQL servers.

## Usage
- `nxc mssql 192.168.178.2 -u path/to/usernames.txt -p path/to/passwords.txt` : search for valid credentials by trying out every combination in the specified lists of usernames and passwords
- `nxc mssql 192.168.178.2 -u username -p password -q 'SELECT * FROM sys.databases;'` : execute the specified SQL query on the target server
- `nxc mssql 192.168.178.2 -u username -p password -x whoami` : execute the specified shell command on the target server through MSSQL
- `nxc mssql 192.168.178.2 -u username -p password -X whoami --no-output` : execute the specified PowerShell command on the target server through MSSQL without retrieving output
- `nxc mssql 192.168.178.2 -u username -p password --get-file C:\path\to\remote_file path/to/local_file` : download a remote file from the target server and store it in the specified location
- `nxc mssql 192.168.178.2 -u username -p password --put-file path/to/local_file C:\path\to\remote_file` : upload a local file to the specified location on the target server

## Categories
files, databases, users

## Links
- More information: https://www.netexec.wiki/mssql-protocol/mssql-passwordspray
- tldr page: https://tldr.inbrowser.app/pages/common/nxc-mssql
