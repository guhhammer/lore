# wg

Gerencia a configuração de interfaces WireGuard.

## Usage
- `sudo wg` : verifica o status das interfaces atualmente ativas
- `sudo wg show interface` : verifica o status da interface especificada
- `wg genkey` : gera uma nova chave privada e a exibe em `stdout`
- `wg <caminho/para/chave_privada pubkey` : gera uma chave pública a partir de uma chave privada e a exibe em `stdout`
- `wg genkey | tee caminho/para/chave_privada | wg pubkey >caminho/para/chave_publica` : gera uma chave pública e uma privada e as salva em arquivos
- `wg genpsk` : gera uma chave pré-compartilhada e a exibe em `stdout`
- `sudo wg showconf interface` : mostra a configuração ativa de uma interface

## Links
- More information: https://www.wireguard.com/quickstart/
