# ipcs

Show information about the usage of XSI IPC facilities: shared memory segments, message queues, and semaphore arrays.

## Usage
- `ipcs -a` : show information about all the IPC
- `ipcs -m|-q|-s` : show information about active shared memory segments, message queues or semaphore sets
- `ipcs -b` : show information on maximum allowable size in bytes
- `ipcs -c` : show creator's user name and group name for all IPC facilities
- `ipcs -p` : show the pID of the last operators for all IPC facilities
- `ipcs -t` : show access times for all IPC facilities
- `ipcs -o` : show outstanding usage for active message queues, and shared memory segments

## See also
- lsipc
- ipcmk
- ipcrm

## Categories
monitoring

## Links
- More information: https://manned.org/ipcs.1p
- tldr page: https://tldr.inbrowser.app/pages/common/ipcs
- Wikipedia: https://en.wikipedia.org/wiki/Ipcs
