# wg-quick

Configura rapidamente túneis WireGuard com base em arquivos de configuração.

## Usage
- `sudo wg-quick up interface` : ativa um túnel VPN com base em `/etc/wireguard/interface.conf`
- `sudo wg-quick down interface` : desativa e remove um túnel VPN
- `sudo wg-quick save interface` : salva a configuração atual de um túnel em `/etc/wireguard/interface.conf`
- `sudo wg-quick strip interface` : exibe a configuração de um túnel removendo as opções específicas do `wg-quick`

## Links
- More information: https://manned.org/wg-quick
