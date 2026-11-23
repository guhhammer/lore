# git rebase

Applica i commit di un ramo su un ramo differente. Tipicamente usato per riallineare (rebase) due rami, creando copie dei commit nella nuova posizione.

## Usage
- `git rebase ramo_della_nuova_base` : riallinea il ramo corrente con il ramo specificato
- `git rebase -i nome_ramo_o_commit_hash` : avvia un rebase interattivo, che consente di riordinare, omettere, unire o modificare i commit
- `git rebase --continue` : prosegui con un rebase che era stato sospeso da un errore di unione, dopo aver risolto i conflitti
- `git rebase --skip` : prosegui con un rebase che era stato sospeso da conflitti di unione, ignorando i commit in conflitto
- `git rebase --abort` : interrompi un rebase in corso (ad esempio perché interrotto da un conflitto di unione)
- `git rebase --onto nuova_base vecchia_base` : sposta parti del ramo corrente su una base differente, specificando la vecchia base di partenza
- `git rebase -i HEAD~5` : applica gli ultimi 5 commit locali, consentendo di riordinarli, ometterli, unirli o modificarli
- `git rebase -X theirs nome_ramo` : risolvi automaticamente i conflitti a favore del ramo di versione corrente (la parola chiave `theirs` ha qui un significato opposto)

## Links
- More information: https://git-scm.com/docs/git-rebase
