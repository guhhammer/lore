# qm stop

Stop a virtual machine.

## Usage
- `qm stop 100` : stop a virtual machine immediately
- `qm stop 100 --timeout 10` : stop a virtual machine and wait for at most 10 seconds
- `qm stop 100 --skiplock true` : stop a virtual machine and skip lock (only root can use this option)
- `qm stop 100 --keepActive true` : stop a virtual machine and don't deactivate storage volumes

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_stop
- tldr page: https://tldr.inbrowser.app/pages/linux/qm-stop
