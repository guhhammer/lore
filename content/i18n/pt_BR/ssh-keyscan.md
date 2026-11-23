# ssh-keyscan

Obter as chaves públicas SSH de servidores remotos.

## Usage
- `ssh-keyscan servidor` : obtém todas as chaves públicas SSH de um servidor remoto
- `ssh-keyscan -p porta servidor` : obtém todas as chaves públicas SSH de um servidor remoto que esteja ouvindo em uma porta específica
- `ssh-keyscan -t rsa,dsa,ecdsa,ed25519 servidor` : obtém determinados tipos de chaves públicas SSH de um servidor remoto
- `ssh-keyscan -H servidor >>~/.ssh/known_hosts` : atualiza manualmente o arquivo known_hosts do SSH com a impressão digital de um determinado servidor

## Links
- More information: https://man.openbsd.org/ssh-keyscan
