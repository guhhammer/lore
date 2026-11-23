# cupsctl

Atualiza ou consulta o cupsd.conf de um server.

## Usage
- `cupsctl` : exibe os valores de configuração atuais
- `cupsctl -h servidor:porta` : exibe os valores de configuração de um servidor específico
- `cupsctl -E` : ativa a criptografia na conexão ao scheduler
- `cupsctl --debug-logging|--no-debug-loging` : ativa ou desativa o registro de depuração para o arquivo error_log
- `cupsctl --remote-admin|--no-remote-admin` : ativa ou desativa administração remota
- `cupsctl | grep '^_debug_logging' | awk -F= '{print $2}'` : exibe o estado atual do registro de depuração

## Links
- More information: https://openprinting.github.io/cups/doc/man-cupsctl.html
