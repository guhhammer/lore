# apt-key

Servizio di gestione delle chiavi per il gestore di pacchetti APT su Debian ed Ubuntu.

## Usage
- `apt-key list` : elenca le chiavi fidate
- `apt-key add file_chiave_pubblica.asc` : aggiunge una chiave al portachiavi delle chiavi fidate
- `apt-key del id_chiave` : elimina una chiave dal portachiavi delle chiavi fidate
- `wget -qO - https://indirizzo.tld/filename.key | apt-key add -` : aggiunge una chiave remota al portachiavi delle chiavi fidate
- `apt-key adv --keyserver pgp.mit.edu --recv id_della_chiave` : aggiunge una chiave da un server di chiavi con il solo ID della chiave

## Links
- More information: https://manned.org/apt-key
