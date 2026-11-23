# journalctl

Consulta o diário do systemd.

## Usage
- `journalctl -n n -f` : mostra as últimas `n` linhas e segue novas mensagens (similar ao `tail --follow` para o syslog tradicional)
- `journalctl -b -1 -p 3` : mostra todas as mensagens com nível de prioridade 3 (erros) da inicialização anterior ao último desligamento
- `journalctl -u unidade` : mostra todas as mensagens de uma unidade específica
- `journalctl _SYSTEMD_INVOCATION_ID=$(systemctl show --value --property=InvocationID unidade)` : mostra logs para uma certa unidade desde a última vez que foi iniciada
- `journalctl -S now|today|yesterday|tomorrow|... -U "AAAA-MM-DD HH:MM:SS"` : filtra mensagens em um período de tempo (por data e hora ou marcadores como "yesterday")
- `journalctl _PID=pid` : mostra todas as mensagens de um processo específico
- `journalctl caminho/para/executável` : mostra todas as mensagens de um executável específico
- `journalctl --vacuum-time 2d` : apaga logs no diário que tenham mais de 2 dias

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/journalctl.html
