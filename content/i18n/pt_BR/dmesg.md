# dmesg

Escreve as mensagens do kernel na terminal.

## Usage
- `sudo dmesg` : exibe as mensagens do kernel
- `sudo dmesg -l err` : exibe as mensagens de erro do kernel
- `sudo dmesg -w` : exibe as mensagens do kernel e manter o terminal esperando por novas menagens, semelhante ao `tail -f` (disponível nas versões 3.5.0 e superiores do kernel)
- `sudo dmesg | grep -i memory` : exibe a quantidade de memória física disponível no sistema
- `sudo dmesg | less` : exibe as mensagens do kernel divididas em páginas
- `sudo dmesg -T` : exibe as menagens do kernel com data/hora (disponível nas versões 3.5.0 e superiores do kernel)
- `sudo dmesg -H` : exibe as mensagens do kernel em um formato de fácil leitura (disponível nas versões 3.5.0 e superiores do kernel)
- `sudo dmesg -L` : exibe as mensagens do kernel utilizando cores (disponível nas versões 3.5.0 e superiores do kernel)

## Links
- More information: https://manned.org/dmesg
