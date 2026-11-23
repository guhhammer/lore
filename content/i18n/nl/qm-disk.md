# qm disk

Beheer schijfimages.

## Usage
- `qm di resi 100 schijf_naam +nG` : voeg `n` gigabytes toe aan een virtuele schijf
- `qm di m 100 scic0 bestemming_opslag_naam` : verplaats een virtuele schijf
- `qm di m 100 scic0 bestemming_opslag_naam --delete` : verwijder de vorige kopie van de virtuele schijf
- `qm di i 100 pad/naar/schijf opslag_naam --format qcow2|raw|vmdk` : importeer een VMDK/`.qcow2`/raw schijfimage met een specifieke opslagnaam
- `qm di resc` : scan alle opslag opnieuw en update schijfgroottes en ongebruikte schijfimages
- `qm di resc --dryrun` : voer een testscan uit en maak geen veranderingen in de configuraties
- `qm di resc --vmid 100` : specificeer een virtuele machine via zijn ID
- `qm di u 100 --idlist unused0,unused1,scsi1,...` : verwijder een schijf

## Links
- More information: https://pve.proxmox.com/pve-docs/qm.1.html#cli_qm_disk_import
