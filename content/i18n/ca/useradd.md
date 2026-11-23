# useradd

Crea un nou usuari.

## Usage
- `sudo useradd nom_usuari` : crea un usuari nou
- `sudo useradd -u id nom_usuari` : crea un usuari amb l'id d'usuari especificada
- `sudo useradd -s ruta/a/la/shell nom_usuari` : crea un usuari nou amb una shell específica
- `sudo useradd -G grup1,grup2 nom_usuari` : crea un usuari nou pertanyent a grups adicionals (cal tenir en compte que no existeixen espais en blanc)
- `sudo useradd -m nom_usuari` : crea un usuari nou amb el directori home predeterminat
- `sudo useradd -k /cami/al/directori_plantilles -m nom_usuari` : crea un usuari nou amb el directori home omplert per una plantilla
- `sudo useradd -r nom_usuari` : crea un usuari nou del sistema sense directori home

## Links
- More information: https://manned.org/useradd
