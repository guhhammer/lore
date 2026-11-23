# ipcmk

Create IPC (Inter-process Communication) resources.

## Usage
- `ipcmk -M segment_size_in_bytes` : create a shared memory segment
- `ipcmk -S element_size` : create a semaphore
- `ipcmk -Q` : create a message queue
- `ipcmk -M segment_size_in_bytes octal_permissions` : create a shared memory segment with specific permissions (default is 0644)

## Categories
processes

## Links
- More information: https://manned.org/ipcmk
- tldr page: https://tldr.inbrowser.app/pages/linux/ipcmk
