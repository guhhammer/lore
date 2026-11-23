# git archive

Creează o arhivă de fișiere dintr-un arbore.

## Usage
- `git archive -v HEAD` : creează o arhivă `.tar` din conținutul `HEAD`-ului curent și o tipărește la `stdout`
- `git archive -v --format zip HEAD` : folosește formatul Zip și raportează progresul în detaliu
- `git archive -v -o cale/către/fișier.zip HEAD` : scrie arhiva Zip într-un fișier specific
- `git archive -o cale/către/fișier.tar nume_ramură` : creează o arhivă `.tar` din conținutul ultimului commit al unei ramuri specifice
- `git archive -o cale/către/fișier.tar HEAD:cale/către/director` : folosește conținutul unui director specific
- `git archive -o cale/către/fișier.tar --prefix cale/către/prefix/ HEAD` : adaugă un prefix de cale la fiecare fișier pentru a-l arhiva în interiorul unui director specific

## Links
- More information: https://git-scm.com/docs/git-archive
