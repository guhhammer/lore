# git send-email

Invia una raccolta di patch via email. Le patch possono essere specificate come file, directory, o liste di revisione.

## Usage
- `git send-email -1` : invia l'ultimo commit nel ramo corrente
- `git send-email -1 commit` : invia un commit specifico
- `git send-email -10` : invia 10 commit nel ramo corrente
- `git send-email -numero_di_commit --compose` : invia un'email con un messaggio introduttivo alla serie di patch
- `git send-email -numero_di_commit --annotate` : revisiona e modifica il messaggio email per ogni patch da inviare

## Links
- More information: https://git-scm.com/docs/git-send-email
