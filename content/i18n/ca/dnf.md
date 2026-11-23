# dnf

Administrador de paquets per RHEL, CentOS i Fedora (Reemplaça a yum).

## Usage
- `sudo dnf update` : actualitza tots els paquets a l'última versió disponible
- `dnf search palabra_clau1 palabra_clau2 ...` : busca un paquet fent servir paraules clau
- `dnf info paquet` : mostra informació sobre un paquet
- `sudo dnf install paquet1 paquet2 ...` : instal·la un nou paquet
- `sudo dnf remove paquet1 paquet2 ...` : desinstal·la un paquet
- `dnf list --installed` : llista tots els paquets instal·lats
- `dnf provides arxiu` : troba quin paquet proveeeix un arxiu determinat

## Links
- More information: https://dnf5.readthedocs.io/en/latest/commands/index.html
