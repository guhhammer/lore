# mount

Krijg toegang tot een volledig bestandssysteem in één map.

## Usage
- `mount` : toon alle gemounte bestandssystemen
- `mount pad/naar/apparaatbestand pad/naar/doelmap` : koppel een apparaat aan een map
- `mount -m pad/naar/apparaatbestand pad/naar/doelmap` : maak een specifieke map aan als deze niet bestaat en koppel er een apparaat aan
- `mount -o uid=gebruiker_id,gid=groep_id pad/naar/apparaatbestand pad/naar/doelmap` : koppel een apparaat aan een map voor een specifieke gebruiker
- `mount -t iso9660 -o ro /dev/cdrom /cdrom` : koppel een CD-ROM-apparaat (met bestandstype ISO9660) aan `/cdrom` (alleen-lezen)
- `mount -a` : koppel alle bestandssystemen die gedefinieerd zijn in `/etc/fstab`
- `mount pad/naar/mountpunt` : koppel een specifiek bestandssysteem omschreven in `/etc/fstab` (b.v. `/dev/sda1 /pad/naar/mountpunt ext2 defaults 0 2`)
- `mount -B pad/naar/oude_map pad/naar/nieuwe_map` : koppel een map aan een andere map

## Links
- More information: https://manned.org/mount.8
