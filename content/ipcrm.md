# ipcrm

Delete IPC (Inter-process Communication) resources.

## Usage
- `ipcrm -m shmem_id` : delete a shared memory segment by ID
- `ipcrm -M shmem_key` : delete a shared memory segment by key
- `ipcrm -q ipc_queue_id` : delete an IPC queue by ID
- `ipcrm -Q ipc_queue_key` : delete an IPC queue by key
- `ipcrm -s semaphore_id` : delete a semaphore by ID
- `ipcrm -S semaphore_key` : delete a semaphore by key
- `ipcrm -a` : delete all IPC resources

## Categories
processes

## Links
- More information: https://manned.org/ipcrm
- tldr page: https://tldr.inbrowser.app/pages/linux/ipcrm
