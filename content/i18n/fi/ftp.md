# ftp

Työkalut File Transfer Protokollan käyttämiseen palvelimen kanssa.

## Usage
- `ftp ftp.example.com` : yhdistä FTP-palvelimeen ja käytä interaktiivisesti
- `ftp ip_osoite portti` : yhdistä FTP-palvelimeen määrittämällä sen IP-osoite ja portti
- `binary` : [Interaktiotila] Vaihda binäärien lähetystilaan (grafiikat, pakatut tiedostot, jne)
- `prompt off` : [Interaktiotila] Siirrä useita tiedostoja kysymättä varmistusta jokaiselle tiedostolle
- `mget *.png` : [Interaktiotila] Lataa useita tiedostoja (`glob`-lauseke)
- `mput *.zip` : [Interaktiotila] Lähetä useita tiedostoja (`glob`-lauseke)
- `mdelete *.txt` : [Interaktiotila] Poista useita tiedostoja etäpalvelimelta
- `rename alkuperäinen_tiedostonimi uusi_tiedostonimi` : [Interaktiotila] Nimeä etäpalvelimella oleva tiedosto uudelleen

## Links
- More information: https://manned.org/ftp
