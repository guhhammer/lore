# bitcoin-cli

Client da linea di comando per interagire con il demone Bitcoin Core tramite chiamate RPC. Utilizza la configurazione definita in `bitcoin.conf`.

## Usage
- `bitcoin-cli sendtoaddress "indirizzo" importo` : invia una transazione a un dato indirizzo
- `bitcoin-cli generate numero_blocchi` : genera uno o più blocchi
- `bitcoin-cli getwalletinfo` : mostra informazioni di alto livello sul portafoglio
- `bitcoin-cli listunspent` : elenca tutti gli output di transazioni precedenti disponibili per finanziare nuove transazioni
- `bitcoin-cli dumpwallet "percorso/del/file"` : esporta le informazioni del portafoglio in un file di testo
- `bitcoin-cli getblockchaininfo` : ottieni informazioni sulla blockchain
- `bitcoin-cli getnetworkinfo` : ottieni informazioni sulla rete
- `bitcoin-cli stop` : ferma il demone Bitcoin Core

## Links
- More information: https://en.bitcoin.it/wiki/Running_Bitcoin#Command-line_arguments
