# beanstalkd

Un semplice e generico gestore di code di lavoro.

## Usage
- `beanstalkd` : avvia beanstalkd, ascoltando sulla porta 11300
- `beanstalkd -l indirizzo_ip -p numero_porta` : avvia beanstalkd ascoltando su porta ed un indirizzo dati
- `beanstalkd -b percorso/della/directory_persistente` : rendi le code di lavoro persistenti salvandole su disco
- `beanstalkd -b percorso/della/directory_persistente -f 500` : sincronizza con una directory persistente ogni 500 millisecondi

## Links
- More information: https://manned.org/beanstalkd
