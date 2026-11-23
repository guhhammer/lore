# pmap

Report memory map of a process or processes.

## Usage
- `pmap process_id` : print memory map for a specific process ID (PID)
- `pmap --extended process_id` : show the extended format
- `pmap --device process_id` : show the device format
- `pmap --range low,high` : limit results to a memory address range specified by `low` and `high`
- `pmap process_id1 process_id2 ...` : print memory maps for multiple processes

## Categories
processes, monitoring

## Links
- More information: https://manned.org/pmap
- tldr page: https://tldr.inbrowser.app/pages/linux/pmap
