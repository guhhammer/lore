# addcomputer.py

Aggiunge un account computer a un dominio. Parte della suite Impacket.

## Usage
- `addcomputer.py -computer-name NOME_COMPUTER$ -computer-pass password_computer dominio/nome_utente:password` : aggiunge un computer con nome e password specifici
- `addcomputer.py -no-add -computer-name NOME_COMPUTER$ -computer-pass password_computer dominio/nome_utente:password` : imposta solo una nuova password su un computer esistente
- `addcomputer.py -delete -computer-name NOME_COMPUTER$ dominio/nome_utente:password` : elimina un account computer esistente
- `addcomputer.py -k -no-pass dominio/nome_utente@hostname` : aggiunge computer usando autenticazione Kerberos
- `addcomputer.py -method LDAPS -port 636 dominio/nome_utente:password` : aggiunge computer via LDAPS (porta 636) invece di SAMR (porta 445)
- `addcomputer.py -dc-host hostname dominio/nome_utente:password` : specifica il domain controller esatto quando esistono più DC

## Links
- More information: https://github.com/fortra/impacket
