# ufw

Firewall Descomplicado. Frontend para `iptables` com o objetivo de facilitar a configuração de um firewall.

## Usage
- `sudo ufw enable` : habilita o ufw
- `sudo ufw disable` : desabilita o ufw
- `sudo ufw status numbered` : mostra regras ufw, juntamente com seus números
- `sudo ufw allow 5432 comment "Service"` : permite tráfego de entrada na porta 5432 nesse host com um que identifique o serviço
- `sudo ufw allow proto tcp from 192.168.0.4 to any port 22` : permite apenas tráfego TCP de 192.168.0.4 pra qualquer endereço deste host, na porta 22
- `sudo ufw deny 80` : nega tráfego na porta 80 desse host
- `sudo ufw deny proto udp from any to any port 8412:8500` : nega todo o tráfego UDP para portas no intervalo 8412:8500
- `sudo ufw delete rule_number` : deleta uma regra particular. O número da regra pode ser recuperado com o `ufw status numbered` comando

## Links
- More information: https://manned.org/ufw
