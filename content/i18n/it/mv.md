# mv

Sposta o rinomina file e directory.

## Usage
- `mv percorso/del/file percorso/di/destinazione` : rinomina un file o una directory quando la destinazione non è una directory esistente
- `mv percorso/di/origine percorso/della/directory_esistente` : sposta un file o una directory in una directory esistente
- `mv percorso/di/origine1 percorso/di/origine2 ... percorso/della/directory_esistente` : sposta più file in una directory esistente, mantenendo i nomi dei file invariati
- `mv -f percorso/di/origine percorso/di/destinazione` : non richiedere conferma prima di sovrascrivere i file esistenti
- `mv -i percorso/di/origine percorso/di/destinazione` : richiedi conferma prima di sovrascrivere i file esistenti, indipendentemente dalle autorizzazioni dei file
- `mv -n percorso/di/origine percorso/di/destinazione` : non sovrascrivere i file esistenti nella destinazione
- `mv -v percorso/di/origine percorso/di/destinazione` : sposta i file in modalità dettagliata, mostrando i file dopo che sono stati spostati

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/mv-invocation.html
