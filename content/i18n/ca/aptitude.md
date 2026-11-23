# aptitude

Eina de gestió de paquets per a Debian i Ubuntu.

## Usage
- `sudo aptitude update` : sincronitza la llista de paquets i versions disponibles (es recomana executar aquest commandament abans que qualsevol altre `aptitude`)
- `sudo aptitude install paquet` : instal·lar un nou paquet i les seves dependències
- `aptitude search paquet` : buscar un paquet
- `aptitude search '?installed(paquet)'` : cercar un paquet instal·lat (`?installed` es un terme de cerca de `aptitude`)
- `sudo aptitude remove paquet` : elimina un paquet i tots els paquets que depenen d'ell
- `sudo aptitude upgrade` : actualitza tots els paquets a les noves versions disponibles
- `sudo aptitude full-upgrade` : actualitza paquets instal·lats (com `aptitude upgrade`), però elimina els paquets obsolets i instal·la paquets nous per satisfer les dependències
- `sudo aptitude hold '?installed(paquete)'` : manté un paquet perquè no sigui actualitzat automàticament

## Links
- More information: https://manned.org/aptitude
