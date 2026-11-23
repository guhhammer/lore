# dpkg-buildpackage

Compila pacchetti Debian binari e/o sorgenti dal codice sorgente. Normalmente eseguito all'interno di un albero sorgente che contiene una directory `debian/`. Gestisce anche le dipendenze di compilazione, crea file come `.buildinfo` e `.changes` e firma il risultato se applicabile.

## Usage
- `dpkg-buildpackage` : genera pacchetti sorgente e binari
- `dpkg-buildpackage -b` : genera solo pacchetti binari (nessun pacchetto sorgente)
- `dpkg-buildpackage -S` : genera solo il pacchetto sorgente (senza compilare binari)
- `dpkg-buildpackage -us -uc` : non firmare i file `.dsc` e `.changes`
- `dpkg-buildpackage -nc` : non eseguire `clean` prima della compilazione
- `dpkg-buildpackage -rfakeroot` : usa `fakeroot` come comando per ottenere privilegi di root durante la compilazione
- `dpkg-buildpackage -Tclean` : esegue un target specifico di `debian/rules`

## Links
- More information: https://manned.org/dpkg-buildpackage
