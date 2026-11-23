# lsipc

Show information on System V IPC facilities currently employed in the system.

## Usage
- `lsipc` : show information about all active IPC facilities
- `lsipc --shmems|--queues|--semaphores` : show information about active shared memory segments, message queues, or semaphore sets
- `lsipc --shmems|--queues|--semaphores -i resource_id` : show full details on the resource with a specific ID
- `lsipc -o KEY,ID,PERMS,SEND,STATUS,NSEMS,RESOURCE,...` : print the given output columns (see all supported columns with `--help`)
- `lsipc --raw|--json|--list|--export` : use raw, JSON, list, or export (key="value") format
- `lsipc --notruncate` : don't truncate the output

## See also
- ipcs

## Links
- More information: https://manned.org/lsipc
- tldr page: https://tldr.inbrowser.app/pages/linux/lsipc
