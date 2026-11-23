# duplicity

Crea archivi incrementali, compressi, cifrati con controllo di versione. Può caricare i backup su una varietà di servizi backend.

## Usage
- `FTP_PASSWORD=password_login_ftp PASSPHRASE=password_cifratura duplicity percorso/della/directory_sorgente ftps://utente@hostname/percorso/della/directory_target/` : esegui il backup di una directory via FTPS su una macchina remota, cifrandolo con una password
- `duplicity --full-if-older-than 1M s3://nome_bucket/prefisso` : esegui il backup di una directory in un server Amazon S3, facendo un backup completo ogni mese
- `FTP_PASSWORD=password_login_webdav duplicity remove-older-than 1Y --force webdav://utente@hostname:porta/directory` : elimina le versioni più vecchie di un anno da un backup salvato in un server WebDAV
- `duplicity collection-status "file://percorso/assoluto/della/directory/di/backup"` : elenca i backup disponibili
- `duplicity list-current-files --time YYYY-MM-DD scp://utente@hostname/percorso/della/directory/backup` : elenca i file in un backup salvato su una macchina remota, via SSH
- `PASSPHRASE=password_chiave_gpg duplicity restore --encrypt-key id_chiave_gpg --path-to-restore percorso/relativo/sotto_directory file://percorso/assoluto/della/directory/di/backup percorso/della/directory/dove/ripristinare` : ripristina una sotto-directory da un backup locale cifrato con GnuPG in una posizione precisa

## Links
- More information: https://duplicity.gitlab.io/stable/duplicity.1.html#name
