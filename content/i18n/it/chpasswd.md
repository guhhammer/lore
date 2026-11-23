# chpasswd

Cambia le password di più utenti utilizzando `stdin`.

## Usage
- `printf "nome_utente:new_password" | sudo chpasswd` : cambia la password di un utente specifico
- `printf "username_1:new_password_1\nusername_2:new_password_2" | sudo chpasswd` : cambia le password di più utenti (il testo in input non deve contenere spazi)
- `printf "nome_utente:new_encrypted_password" | sudo chpasswd -e` : cambia la password di un utente specifico, specificandola in forma crittografata
- `printf "nome_utente:new_password" | sudo chpasswd -c NONE|DES|MD5|SHA256|SHA512` : cambia la password di un utente specifico e utilizza un metodo di crittografia specifico

## Links
- More information: https://manned.org/chpasswd
