# git imerge

Esegui un'unione (merge) o rebase tra due rami Git in modo incrementale. Eventuali conflitti tra i due rami sono tracciati in coppie di commit distinti, per semplificarne la risoluzione.

## Usage
- `git imerge rebase ramo_su_cui_eseguire_il_rebase` : avvia un rebase usando imerge (dopo aver fatto checkout sul ramo da spostare)
- `git imerge merge ramo_da_unire` : avvia un'unione usando imerge (dopo aver fatto checkout sul ramo di destinazione)
- `git imerge diagram` : mostra con un diagramma ASCII lo stato di esecuzione dell'unione o rebase
- `git imerge continue --no-edit` : continua con l'operazione di imerge dopo aver risolto i conflitti (dopo aver aggiunto i file in conflitto con `git add`)
- `git imerge finish` : concludi l'operazione di imerge dopo aver risolto tutti i conflitti
- `git imerge remove && git checkout ramo_precedente` : interrompi l'operazione di imerge e ritorna al ramo precedente

## Links
- More information: https://github.com/mhagger/git-imerge
