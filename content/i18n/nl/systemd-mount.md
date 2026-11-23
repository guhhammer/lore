# systemd-mount

Maak tijdelijke mount- of auto-mountpunten aan en verwijder ze.

## Usage
- `systemd-mount pad/naar/bestand_of_apparaat` : mount een bestandssysteem (image of blokapparaat) op `/run/media/system/LABEL` waar LABEL het bestandssysteemlabel is of de apparaatnaam als er geen label is
- `systemd-mount pad/naar/bestand_of_apparaat pad/naar/mountpunt` : mount een bestandssysteem (image of blokapparaat) op een gegeven locatie
- `systemd-mount --list` : toon een lijst van alle lokale, bekende blokapparaten met de bestandssystemen die mogelijk gemount kunnen worden
- `systemd-mount --automount yes pad/naar/bestand_of_apparaat` : maak een auto-mountpunt dat het daadwerkelijke bestandssysteem zal mounten op het moment van eerste toegang
- `systemd-mount -u pad/naar/mountpunt_of_apparaat1 pad/naar/mountpunt_of_apparaat2 ...` : unmount een of meerdere apparaten
- `systemd-mount -t bestandssysteemtype pad/naar/bestand_of_apparaat pad/naar/mountpunt` : mount een bestandssysteem (image of blokapparaat) met een specifiek bestandssysteemtype
- `systemd-mount -o mount_opties pad/naar/bestand_of_apparaat pad/naar/mountpunt` : mount een bestandssysteem (image of blokapparaat) met extra mount-opties

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemd-mount.html
