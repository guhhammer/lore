# limactl

Virtuele machinebeheerder voor Linux gasten, met meerdere VM-templates beschikbaar. Kan worden gebruikt om containers op macOS uit te voeren, maar ook voor generieke virtuele machine gebruiksscenario's op macOS en Linux hosts.

## Usage
- `limactl list` : toon VMs
- `limactl create --name vm_naam template:debian|fedora|ubuntu|...` : maak een VM met standaard instellingen en voorzie optioneel van een naam en/of template (zie `limactl create --list-templates` voor beschikbare templates)
- `limactl start vm_naam` : start een VM (dit kan enkele afhankelijkheden erin installeren en een paar minuten duren)
- `limactl shell vm_naam` : open een externe shell in een VM
- `limactl shell vm_naam commando` : voer een commando uit in een VM
- `limactl stop vm_naam` : stop/sluit een VM af
- `limactl remove vm_naam` : verwijder een VM

## Links
- More information: https://github.com/lima-vm/lima
