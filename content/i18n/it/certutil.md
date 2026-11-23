# certutil

Gestisce chiavi e certificati sia nei database NSS che in altri token NSS.

## Usage
- `certutil -N -d .` : crea un Nuovo database di certificati nella directory corrente
- `certutil -L -d .` : lista tutti i certificati presenti in un database
- `certutil -K -d . -f percorso/del/file_password.txt` : elenca tutte le Key private in un database specificando il file della password
- `certutil -A -n "certificato_server" -t ",," -i percorso/del/file.crt -d .` : aggiunge il certificato firmato al database del richiedente specificando un nickname, gli attributi di trust e un input file CRT
- `certutil -S -f percorso/del/file_password.txt -d . -t ",," -c "certificato_server" -n "nome_server" -g 2048 -s "CN=common_name,O=organization"` : aggiunge nomi alternativi del soggetto a un dato certificate con una specifica dimensione della chiave (g)

## Links
- More information: https://manned.org/certutil
