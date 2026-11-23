# rankmirrors

Classifica una lista di mirror di Pacman in base alla velocità di connessione e apertura. Scrive la nuova lista di mirror su `stdout`.

## Usage
- `rankmirrors /etc/pacman.d/mirrorlist` : classifica una lista di mirror
- `rankmirrors -n numero /etc/pacman.d/mirrorlist` : mostra solo un certo numero di server con il punteggio più alto
- `rankmirrors -v /etc/pacman.d/mirrorlist` : mostra output dettagliato durante la generazione della lista di mirror
- `rankmirrors -u url` : verifica solo un URL specifico
- `rankmirrors -t /etc/pacman.d/mirrorlist` : mostra solo i tempi di risposta invece della lista completa

## Links
- More information: https://manned.org/rankmirrors
