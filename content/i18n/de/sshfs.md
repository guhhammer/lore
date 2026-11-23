# sshfs

Dateisystem Client für SSH.

## Usage
- `sshfs benutzer@externer_server:externes_verzeichnis lokales_einhänge_verzeichnis` : hänge ein externes Verzeichnis ein
- `umount lokaler_einhänge_verzeichnis` : hänge ein externes Verzeichnis aus
- `sshfs benutzer@externer_server:externes_verzeichnis -p 2222` : hänge ein externes Verzeichnis unter einem bestimmten Port ein
- `sshfs benutzer@externer_server:externes_verzeichnis -C` : verwende Komprimierung
- `sshfs -o follow_symlinks benutzer@externer_server:externes_verzeichnis lokaler_einhänge_verzeichnis` : folge symbolischen Links

## Links
- More information: https://github.com/libfuse/sshfs/blob/master/sshfs.rst
