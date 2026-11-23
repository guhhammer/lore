# setcap

Set capabilities of specified file.

## Usage
- `setcap 'cap_net_raw' path/to/file` : set capability `cap_net_raw` (to use RAW and PACKET sockets) for a given file
- `setcap 'cap_dac_read_search,cap_sys_tty_config+ep' path/to/file` : set multiple capabilities on a file (`ep` behind the capability means "effective permitted")
- `setcap -r path/to/file` : remove all capabilities from a file
- `setcap -v 'cap_net_raw' path/to/file` : verify that the specified capabilities are currently associated with the specified file
- `setcap -n root_uid 'cap_net_admin' path/to/file` : set the file capability for use only in a user namespace with a specific root user ID owner

## See also
- getcap

## Categories
files, network, shell

## Links
- More information: https://manned.org/setcap
- tldr page: https://tldr.inbrowser.app/pages/linux/setcap
