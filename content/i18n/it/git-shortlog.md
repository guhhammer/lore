# git shortlog

Riassume l'output di `git log`.

## Usage
- `git shortlog` : mostra un riassunto dei commit fatti, raggruppati alfabeticamente per nome dell'autore
- `git shortlog -n` : mostra un riassunto dei commit fatti, ordinati per numero di commit
- `git shortlog -c` : mostra un riassunto dei commit fatti, raggruppati per identità dell'utente che ha eseguito il commit (nome e email)
- `git shortlog HEAD~5..HEAD` : mostra un riassunto degli ultimi 5 commit (richiesti sottoforma di intervallo di revisione)
- `git shortlog -sne` : mostra tutti gli utenti, email e numero di commit nel ramo corrente
- `git shortlog -sne --all` : mostra tutti gli utenti, email e numero di commit in tutti i rami

## Links
- More information: https://git-scm.com/docs/git-shortlog
