# qm wait

Wait until the virtual machine is stopped.

## Usage
- `qm w 100` : wait until the virtual machine is stopped
- `qm w 100 --timeout 10` : wait until the virtual machine is stopped with a 10 second timeout
- `qm shu 100 && qm w 100 --timeout 10` : send a shutdown request, then wait until the virtual machine is stopped with a 10 second timeout

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_wait
- tldr page: https://tldr.inbrowser.app/pages/linux/qm-wait
