# lftp

Sophisticated file transfer program.

## Usage
- `lftp -u username ftp.example.com` : connect to an FTP server
- `mget path/to/*.png` : [Interactive] Download multiple files (`glob` expression)
- `mput path/to/*.zip` : [Interactive] Upload multiple files (`glob` expression)
- `mrm path/to/*.txt` : [Interactive] Delete multiple files on the remote server
- `mv original_filename new_filename` : [Interactive] Rename a file on the remote server
- `mirror path/to/remote_directory path/to/local_output_directory` : [Interactive] Download or update an entire directory
- `mirror -R path/to/local_directory path/to/remote_output_directory` : [Interactive] Upload or update an entire directory

## Categories
files, network

## Links
- More information: https://lftp.yar.ru/lftp-man.html
- tldr page: https://tldr.inbrowser.app/pages/linux/lftp
