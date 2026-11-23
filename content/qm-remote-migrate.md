# qm remote-migrate

Migrate a virtual machine to a remote Proxmox host or cluster.

## Usage
- `qm remote-migrate vmid target_vmid 'apitoken=PVEAPIToken=user@realm!token=secret,host=address,fingerprint=fingerprint' --target-bridge bridge --target-storage storage` : migrate a VM to a remote host
- `qm remote-migrate vmid target_vmid 'apitoken=PVEAPIToken=user@realm!token=secret,host=address,fingerprint=fingerprint' --target-bridge bridge --target-storage storage --online` : migrate a running VM with live migration
- `qm remote-migrate vmid target_vmid 'apitoken=PVEAPIToken=user@realm!token=secret,host=address,fingerprint=fingerprint' --target-bridge bridge --target-storage storage --delete 1` : delete the source VM after a successful migration
- `qm remote-migrate vmid target_vmid 'apitoken=PVEAPIToken=user@realm!token=secret,host=address,fingerprint=fingerprint' --target-bridge bridge --target-storage storage --bwlimit value` : limit migration bandwidth
- `qm remote-migrate vmid target_vmid 'apitoken=PVEAPIToken=user@realm!token=secret,host=address,fingerprint=fingerprint' --target-bridge 1 --target-storage 1` : use the same bridge and storage names on the target
- `qm remote-migrate vmid target_vmid 'apitoken=PVEAPIToken=user@realm!token=secret,host=address,fingerprint=fingerprint,port=porta' --target-bridge bridge --target-storage storage` : connect to a remote API on a custom port

## Categories
network, users, containers

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_remote-migrate
- tldr page: https://tldr.inbrowser.app/pages/linux/qm-remote-migrate
