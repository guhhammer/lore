# usermod

Modifica una conta d'usuari.

## Usage
- `sudo usermod -l nou_nom_usuari nom_usuari` : canvia el nom d'usuari
- `sudo usermod -u id nom_usuari` : canvia l'id d'usuari
- `sudo usermod -s cami/a/shell nom_usuar` : canvia la shell d'un usuari
- `sudo usermod -a -G grup1,grup2 nom_usuar` : afegeix un usuari a grups suplementaris (cal tenir en compte els espais en blanc)
- `sudo usermod -m -d ruta/al/home nom_usuar` : crea un nou directori home per un usuari i mou tots els arxius a ell

## Links
- More information: https://manned.org/usermod
