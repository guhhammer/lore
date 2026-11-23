# qm guest

Beheer een VM-gastagent.

## Usage
- `qm g exec-s 100 proces_id` : toon de status van een specifieke PID
- `qm g p 100 gebruikersnaam` : stel interactief een wachtwoord in voor een specifieke gebruiker in een virtuele machine
- `qm g p 100 gebruikersnaam --crypted 1` : stel interactief een reeds gehasht wachtwoord in voor een specifieke gebruiker in een virtuele machine
- `qm g c 100 fsfreeze-freeze|fsfreeze-status|fsfreeze-thaw|fstrim|get-fsinfo|...` : voer een specifiek QEMU Guest Agent-commando uit
- `qm g exec 100 commando argument1 argument2 ...` : voer een specifiek commando uit via een gastagent
- `qm g exec 100 argument1 argument2 ... --synchronous 0` : voer een specifiek commando asynchroon uit via een gastagent
- `qm g exec 100 argument1 argument2... --timeout 10` : voer een specifiek commando uit via een gastagent met een opgegeven time-out van 10 seconden
- `qm g exec 100 argument1 argument2 ... --pass-stdin 1` : voer een specifiek commando uit via een gastagent en stuur invoer van `stdin` tot EOF door naar de gastagent

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_guest_cmd
