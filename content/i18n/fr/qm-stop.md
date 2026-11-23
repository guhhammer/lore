# qm stop

Arrête une machine virtuelle.

## Usage
- `qm stop 100` : arrête immédiatement une machine virtuelle
- `qm stop 100 --timeout 10` : arrête une machine virtuelle et attend au maximum 10 secondes
- `qm stop 100 --skiplock true` : arrête une machine virtuelle et ignore le verrou (seul l'utilisateur root peut utiliser cette option)
- `qm stop 100 --keepActive true` : arrête une machine virtuelle sans désactiver les volumes de stockage

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_stop
