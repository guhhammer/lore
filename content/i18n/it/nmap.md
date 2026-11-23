# nmap

Tool per esplorazione di rete e scansione porte/sicurezza. Alcune funzionalità (es. SYN scan) si attivano solo con privilegi root.

## Usage
- `nmap -v1|2|3 ip_o_nome_host` : scansiona le prime 1000 porte di un host remoto con vari livelli di verbosità
- `nmap -T5 -sn 192.168.0.0/24|ip_o_nome_host1,ip_o_nome_host2,...` : esegui ping sweep aggressivo su subnet intera o host individuali
- `sudo nmap -A -iL percorso/del/file.txt` : abilita rilevamento OS, versione, script scanning e traceroute da file
- `nmap -p porta1,porta2,... ip_o_host1,ip_o_host2,...` : scansiona lista specifica di porte (usa `-p-` per tutte le porte da 1 a 65535)
- `nmap -sC -sV -oA top-1000-ports ip_o_host1,ip_o_host2,...` : rileva servizi e versioni sulle prime 1000 porte con script NSE di default, salva risultati (`-oA`)
- `nmap --script "default and safe" ip_o_host1,ip_o_host2,...` : scansiona target con cura usando script NSE `default and safe`
- `nmap --script "http-*" ip_o_host1,ip_o_host2,... -p 80,443` : scansiona web server su porte standard porte 80 e 443 con tutti gli script `http-*` NSE
- `sudo nmap -T0 -D decoy_ip1,decoy_ip2,... --source-port 53 -f --data-length 16 -Pn ip_o_host` : tenta evasione IDS/IPS con scansione lentissima (`-T0`), Decoy indirizzi sorgente, pacchetti frammentati, dati casuali

## Links
- More information: https://nmap.org/book/man.html
