# adduser

Servizio per aggiungere utenti.

## Usage
- `adduser nome_utente` : crea un nuovo utente con una directory home predefinita e richiede all'utente di impostare una password
- `adduser --no-create-home nome_utente` : crea un utente senza una directory home
- `adduser --home percorso/della/home nome_utente` : crea un utente con una directory home nel percorso specificato
- `adduser --shell percorso/della/shell nome_utente` : crea un nuovo utente con l'interprete di comandi(shell) specificato come shell di accesso
- `adduser --ingroup gruppo nome_utente` : crea un nuovo utente appartenente al gruppo specificato
- `adduser nome_utente gruppo` : aggiunge un utente esistente al gruppo specificato

## Links
- More information: https://manned.org/adduser
