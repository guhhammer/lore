# secon

Krijg de SELinux-beveiligingscontext van een bestand, PID, huidige uitvoeringscontext of een contextspecificatie.

## Usage
- `secon` : krijg de beveiligingscontext van de huidige uitvoeringscontext
- `secon -p 1` : krijg de huidige beveiligingscontext van een proces
- `secon -f pad/naar/bestand_of_map` : krijg de huidige beveiligingscontext van een bestand, waarbij alle tussenliggende symlinks worden opgelost
- `secon -L pad/naar/symlink` : krijg de huidige beveiligingscontext van een symlink zelf (d.w.z. niet oplossen)
- `secon system_u:system_r:container_t:s0:c899,c900` : parse en leg een contextspecificatie uit

## Links
- More information: https://manned.org/secon
