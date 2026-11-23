# haveged

Hardware-based random number generator.

## Usage
- `sudo haveged` : generate a random number
- `sudo haveged -F` : run `haveged` in foreground
- `sudo haveged -f path/to/file` : set file path for output of `haveged`
- `sudo haveged -r runlevel` : set run level for daemon
- `sudo haveged -b buffer_size_in_kw` : set collection buffer size in kibibyte words
- `sudo haveged -c command` : insert a command to an already running `haveged` process or daemon
- `sudo haveged -d cache_size_in_kw` : set cache size in kibibyte words
- `sudo haveged -n byteamount` : set number of bytes to write to the output file

## Categories
users, files

## Links
- More information: https://manned.org/haveged
- tldr page: https://tldr.inbrowser.app/pages/linux/haveged
