# ip

Mostra / manipula roteamento, dispositivos, roteamento baseado em póliticas e túneis. Alguns subcomandos como `address` têm suas pŕoprias documentações de uso.

## Usage
- `ip a` : lista interfaces com informações detalhadas
- `ip -br a` : lista interfaces com breves informações sobre a camada de rede
- `ip -br l` : lista interfaces com breves informações sobre a camada de link de dados
- `ip r` : exibe a tabela de roteamento
- `ip n` : mostra vizinhos (ARP tabela)
- `sudo ip l s interface up|down` : ativa / desativa uma interface
- `sudo ip a add|delete ip_address/mask dev interface` : adiciona / remove um endereço de IP a uma interface
- `sudo ip r a default via ip_address dev interface` : adiciona uma rota padrão

## Links
- More information: https://manned.org/ip.8
