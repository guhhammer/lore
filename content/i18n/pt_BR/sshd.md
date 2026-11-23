# sshd

Daemon do Secure Shell - permite que máquinas remotas façam login de forma segura na máquina atual. Máquinas remotas podem executar comandos como se estivessem sendo executados nesta máquina.

## Usage
- `sshd` : inicia o daemon em segundo plano
- `sshd -D` : executa o sshd em primeiro plano
- `sshd -D -d` : executa com saída detalhada (para depuração)
- `sshd -p porta` : executa em uma porta específica

## Links
- More information: https://man.openbsd.org/sshd
