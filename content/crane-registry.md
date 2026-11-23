# crane registry

This command serves a registry implementation on an automatically chosen port (`:0`), `$PORT`, or `--address`. The command blocks while the server accepts pushes and pulls and contents are can be stored in memory, and disk.

## Usage
- `crane registry serve` : serve a registry implementation
- `crane registry serve --address address_name` : specify the address to listen on
- `crane registry serve --disk path/to/directory` : store blobs in a specific directory
- `crane registry -h` : display help for `crane registry`
- `crane registry serve -h` : display help for `crane registry serve`

## Categories
network, disks, files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/crane-registry
