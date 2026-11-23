# qm snapshot

Create virtual machine snapshots.

## Usage
- `qm sn 100 snapshot_name` : create a snapshot of a specific virtual machine (name must start with a letter)
- `qm sn 100 snapshot_name --description description` : create a snapshot with a specific description
- `qm sn 100 snapshot_name --description description --vmstate 1` : create a snapshot including the running virtual machine state
- `qm lists 100` : list snapshots of a VM
- `qm ro 100 snapshot_name` : roll back the state of a specific VM to a specified snapshot

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_snapshot
- tldr page: https://tldr.inbrowser.app/pages/linux/qm-snapshot
