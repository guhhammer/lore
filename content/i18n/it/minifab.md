# minifab

Strumento per semplificare il settaggio e il deployment di una blockchain Hyperledger Fabric.

## Usage
- `minifab up -i versione_minifab` : crea la blockchain Hyperledger Fabric
- `minifab down` : rimuovi la blockchain Hyperledger Fabric
- `minifab install -n nome_smart_contract` : installa smart contract su un canale
- `minifab install -n nome_smart_contract -v versione_smart_contract` : installa smart contract su un canale specificando la versione
- `minifab approve,commit,initialize,discover` : inizializza smart contract dopo installazione/aggiornamento dello stesso
- `minifab invoke -n nome_smart_contract -p '"nome_metodo", "arg0", "arg1", ...'` : interroga smart contract con argomenti
- `minifab blockquery numero_blocco` : interroga la blockchain
- `minifab apprun -l linguaggio_di_programmazione` : esegui direttamente l'applicazione

## Links
- More information: https://github.com/hyperledger-labs/minifabric
