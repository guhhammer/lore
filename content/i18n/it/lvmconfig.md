# lvmconfig

Visualizza e gestisce le informazioni di configurazione di LVM.

## Usage
- `lvmconfig --typeconfig current --mergedconfig` : mostra la configurazione effettiva in uso (dopo aver unito tutte le sorgenti di configurazione)
- `lvmconfig --typeconfig diff` : mostra solo le impostazioni che differiscono dai valori predefiniti
- `lvmconfig -l` : elenca tutte le chiavi di configurazione
- `lvmconfig --typeconfig default --withcomments --withspaces` : stampa la configurazione predefinita con commenti completi e spaziatura aggiuntiva
- `lvmconfig --mergedconfig --validate` : convalida la configurazione completa unita e segnala gli errori
- `lvmconfig --typeconfig current -f path/to/output.conf` : scrivi la configurazione effettiva corrente in un file

## Links
- More information: https://manned.org/lvmconfig
