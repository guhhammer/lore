# borg

Strumento di backup con deduplicazione. Crea backup locali o remoti che sono montabili come filesystem.

## Usage
- `borg init -e repokey|keyfile|none|... /percorso/del/repo_o_directory` : inizializza una repository (locale)
- `borg create --progress /percorso/del/repo_o_directory::Lunedi /percorso/della/directory_sorgente` : esegui il backup di una directory nella repository, creando un archivio chiamato "Lunedi"
- `borg list /percorso/del/repo_o_directory` : lista tutti gli archivi in una repository
- `borg extract utente@host:/percorso/del/repo_o_directory::Lunedi percorso/della/directory_destinazione --exclude '*.ext'` : estrai una specifica directory dall'archivio "Lunedi" in una repository remota, escludendo tutti i file `.ext`
- `borg prune --keep-within 7d --list /percorso/del/repo_o_directory` : riduci una repository eliminando tutti gli archivi più vecchi di 7 giorni, elencando i cambiamenti
- `borg mount /percorso/del/repo_o_directory::Lunedi /percorso/del/mountpoint` : monta una repository come filesystem FUSE
- `borg create --help` : mostra aiuto sul come creare archivi

## Links
- More information: https://borgbackup.readthedocs.io/en/stable/usage/general.html
