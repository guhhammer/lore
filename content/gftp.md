# gftp

This command is an alias of GNU `ftp`.

## Usage
- `ftp ftp.example.com` : connect to an FTP server and run in interactive mode
- `ftp ip_address port` : connect to an FTP server specifying its IP address and port
- `binary` : [Interactive] Switch to binary transfer mode (graphics, compressed files, etc.)
- `prompt off` : [Interactive] Transfer multiple files without prompting for confirmation on every file
- `mget *.png` : [Interactive] Download multiple files (`glob` expression)
- `mput *.zip` : [Interactive] Upload multiple files (`glob` expression)
- `mdelete *.txt` : [Interactive] Delete multiple files on the remote server
- `rename original_filename new_filename` : [Interactive] Rename a file on the remote server

## Categories
network, files, shell, macos

## Links
- tldr page: https://tldr.inbrowser.app/pages/osx/gftp
