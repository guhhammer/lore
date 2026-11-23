# lsipc

Toon informatie over System V IPC-faciliteiten die momenteel in gebruik zijn op het systeem.

## Usage
- `lsipc` : toon informatie over alle actieve IPC-faciliteiten
- `lsipc --shmems|--queues|--semaphores` : toon informatie over actieve gedeelde memory-segmenten, berichtenqueues of semaphore-sets
- `lsipc --shmems|--queues|--semaphores -i resource_id` : toon volledige details over de resource met een specifieke ID
- `lsipc -o KEY,ID,PERMS,SEND,STATUS,NSEMS,RESOURCE,...` : toon de opgegeven outputkolommen (bekijk alle ondersteunde kolommen met `--help`)
- `lsipc --raw|--json|--list|--export` : gebruik het raw, JSON, list, of export (key="value") formaat
- `lsipc --notruncate` : kap de output niet af

## Links
- More information: https://manned.org/lsipc
