# mimikatz crypto

Manipola i servizi crittografici di Windows e i certificati, consentendo di elencare ed esportare certificati e chiavi, anche se contrassegnati come non esportabili. Richiede in genere privilegi elevati, soprattutto quando si accede a chiavi di sistema.

## Usage
- `mimikatz "crypto::providers"` : elenca i provider crittografici (CryptoAPI/CNG) disponibili
- `mimikatz "crypto::capi"` : elenca le chiavi in un provider crittografico (e applica patch per rendere esportabili le chiavi non esportabili tramite CryptoAPI)
- `mimikatz "crypto::certificates /export"` : esporta certificati e chiavi dai negozi certificati

## Links
- More information: https://github.com/gentilkiwi/mimikatz
