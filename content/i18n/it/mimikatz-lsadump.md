# mimikatz lsadump

Esegue il dump di segreti dalla Windows Local Security Authority (LSA). Richiede privilegi SYSTEM.

## Usage
- `mimikatz "lsadump::sam"` : esegue il dump degli hash SAM
- `mimikatz "lsadump::secrets"` : esegue il dump dei segreti dall'hive SECURITY
- `mimikatz "lsadump::cache"` : esegue il dump delle credenziali di dominio memorizzate in cache

## Links
- More information: https://github.com/gentilkiwi/mimikatz
