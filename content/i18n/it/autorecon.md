# autorecon

Uno strumento multi-threaded di ricognizione di rete che esegue l'enumerazione automatica dei servizi.

## Usage
- `sudo autorecon host_o_ip1,host_o_ip2,...` : esegui la ricognizione su uno o più host di destinazione (i risultati dettagliati della scansione verranno salvati in `./results`)
- `sudo autorecon -t percorso/del/file` : esegui la ricognizione su target specificati in un file
- `sudo autorecon -o percorso/dei/risultati host_o_ip1,host_o_ip2,...` : salva i risultati in una directory diversa
- `sudo autorecon -p T:21-25,80,443,U:53,B:123 host_o_ip1,host_o_ip2,...` : limita la scansione a porte e protocolli specifici (`T` per TCP, `U` per UDP, `B` per entrambi)

## Links
- More information: https://github.com/AutoRecon/AutoRecon
