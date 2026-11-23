# pveam

Manage LXC container templates.

## Usage
- `pveam u` : update container template database
- `pveam a` : list available templates
- `pveam d local template_name` : download a template
- `pveam l local` : list downloaded templates
- `pveam a --section system|turnkeylinux|mail` : list available templates in a specific section
- `pveam r local:vztmpl/template_name` : remove a template

## Categories
containers, network

## Links
- More information: https://pve.proxmox.com/pve-docs/pveam.1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/pveam
