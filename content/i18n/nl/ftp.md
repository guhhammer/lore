# ftp

Hulpmiddelen om via het File Transfer Protocol met een server te communiceren.

## Usage
- `ftp ftp.example.com` : verbind met een FTP-server en start in interactieve modus
- `ftp ip_adres poort` : verbind met een FTP-server met opgave van IP-adres en poort
- `binary` : [Interactief] Schakel om naar binaire overdrachtsmodus (grafische bestanden, gecomprimeerde bestanden, etc)
- `prompt off` : [Interactief] Draag meerdere bestanden over zonder bevestiging voor elk bestand
- `mget *.png` : [Interactief] Download meerdere bestanden (`glob`-expressie)
- `mput *.zip` : [Interactief] Upload meerdere bestanden (`glob`-expressie)
- `mdelete *.txt` : [Interactief] Verwijder meerdere bestanden op de externe server
- `rename originele_bestandsnaam nieuwe_bestandsnaam` : [Interactief] Hernoem een bestand op de externe server

## Links
- More information: https://manned.org/ftp
