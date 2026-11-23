# git bisect

Usa la ricerca binaria per trovare il commit che ha introdotto un bug. Git salta automaticamente avanti ed indietro nell'albero dei commit per restringere progressivamente il campo fino al commit colpevole.

## Usage
- `git bisect start commit_cattivo commit_buono` : avvia una ricerca su un intervallo di commit definito dal commit "cattivo" contenente il bug ed un altro commit "buono" privo del bug (solitamente più vecchio)
- `git bisect good|bad` : contrassegna ogni commit selezionato da `git bisect` come "bad" (cattivo) o "good" (buono) dopo averlo testato per verificare la presenza del bug
- `git bisect reset` : una volta che `git bisect` ha individuato il commit che ha introdotto il bug, termina la sessione di ricerca e torna al ramo precedente
- `git bisect skip` : ignora un commit durante la ricerca (ad esempio uno che fallisce i test per un motivo diverso dal bug ricercato)

## Links
- More information: https://git-scm.com/docs/git-bisect
