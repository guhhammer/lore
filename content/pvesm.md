# pvesm

Manage Proxmox storage.

## Usage
- `pvesm st` : get status for all datastores
- `pvesm l storage_name` : list storage contents
- `pvesm pa storage_name:content_type/file_name` : view the path of a stored file
- `pvesm ad dir storage_name --path path/to/directory` : add a directory storage
- `pvesm se storage_name --content iso,images,backup,vztmpl,rootdir,snippets,import` : set a storage to contain specific content
- `pvesm f local:iso/archlinux-2025.08.01-x86_64.iso` : delete a file from storage
- `pvesm r storage_name` : remove a storage

## Categories
files

## Links
- More information: https://pve.proxmox.com/pve-docs/pvesm.1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/pvesm
