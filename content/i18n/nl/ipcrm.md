# ipcrm

Verwijder IPC-resources (Inter-process Communication).

## Usage
- `ipcrm -m shmem_id` : verwijder een gedeeld geheugensegment op basis van ID
- `ipcrm -M shmem_key` : verwijder een gedeeld geheugensegment op basis van key
- `ipcrm -q ipc_queue_id` : verwijder een IPC-wachtrij op basis van ID
- `ipcrm -Q ipc_queue_key` : verwijder een IPC-wachtrij op basis van key
- `ipcrm -s semaphore_id` : verwijder een semafoor op basis van ID
- `ipcrm -S semaphore_key` : verwijder een semafoor op basis van key
- `ipcrm -a` : verwijder alle IPC-resources

## Links
- More information: https://manned.org/ipcrm
