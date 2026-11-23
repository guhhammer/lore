# nmap

Ferramenta de exploração de redes e scanner de segurança/portas. Algumas funcionalidades (ex. scan SYN) só funcionam quando o `nmap` é executado com privilégios root.

## Usage
- `nmap -v1|2|3 ip_ou_hostname` : escaneia as 1000 portas mais comuns de um host remoto com vários níveis de verbosidade
- `nmap -T5 -sn 192.168.0.0/24|ip_ou_hostname1,ip_ou_hostname2,...` : executa uma varredura de ping em uma sub-rede ou servidores individuais agressivamente
- `sudo nmap -A -iL caminho_para_arquivo.txt` : ativa detecção de OS, detecção de versão, escaneamento por script e traceroute de servidores a partir de um arquivo
- `nmap -p porta1,porta2, ... ip_ou_host1,ip_ou_host2,...` : escaneia uma lista específica de portas (Usa `-p-` para todas as portas de 1 a 65535)
- `nmap -sC -sV -oA 1000-primeiras-portas ip_ou_host1,ip_ou_host2,...` : executa detecção de serviço e versão das 1000 portas mais comuns usando scripts NSE padrão, salvando os resultados (`-oA`) em arquivos de saída
- `nmap --script "default and safe" ip_ou_host1,ip_ou_host2,...` : escaneia o(s) alvo(s) cuidadosamente usando scripts NSE `default and safe`
- `nmap --script "http-*" ip_ou_host1,ip_ou_host2,... -p 80,443` : procura servidores web rodando nas portas padrão 80 e 443 usando todos os scripts NSE `http-*` disponíveis
- `sudo nmap -T0 -D ip_falso1,ip_falso2, ... --source-port 53 -f --data-length 16 -Pn ip_ou_host` : tenta evadir detecção IDS/IPS usando um scan extremamente lento (`-T0`), endereços de origem falsos (`-D`), pacotes fragmentados, dados aleatórios e outros métodos

## Links
- More information: https://nmap.org/book/man.html
