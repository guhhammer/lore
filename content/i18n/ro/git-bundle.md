# git bundle

Împachetează obiecte și referințe într-o arhivă.

## Usage
- `git bundle create cale/către/fișier.bundle nume_ramură` : creează un fișier bundle care conține toate obiectele și referințele unei ramuri specifice
- `git bundle create cale/către/fișier.bundle --all` : creează un fișier bundle pentru toate ramurile
- `git bundle create cale/către/fișier.bundle -5 HEAD` : creează un fișier bundle cu ultimele 5 commit-uri ale ramurii curente
- `git bundle create cale/către/fișier.bundle --since 7.days HEAD` : creează un fișier bundle cu ultimele 7 zile
- `git bundle verify cale/către/fișier.bundle` : verifică dacă un fișier bundle este valid și poate fi aplicat la repository-ul curent
- `git bundle unbundle cale/către/fișier.bundle` : tipărește în `stdout` lista de referințe conținute într-un bundle
- `git pull cale/către/fișier.bundle nume_ramură` : despachetează o ramură specifică dintr-un fișier bundle în repository-ul curent
- `git clone cale/către/fișier.bundle` : creează un repository nou dintr-un bundle

## Links
- More information: https://git-scm.com/docs/git-bundle
