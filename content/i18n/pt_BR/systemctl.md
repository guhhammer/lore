# systemctl

Controla o sistema systemd e o gerenciador de serviços.

## Usage
- `systemctl status` : mostra todos os serviços em execução
- `systemctl --failed` : lista unidades com falha
- `systemctl start|stop|restart|reload unidade` : inicia/Para/Reinicia/Recarrega o estado um serviço
- `systemctl enable|disable unidade` : ativa/Desativa uma unidade a ser iniciada na inicialização
- `systemctl daemon-reload` : recarrega o systemd, verificando por unidades novas ou alteradas
- `systemctl is-active|is-enabled|isfailed unidade` : verifica se uma unidade está ativada/ativa/em falha
- `systemctl list-units -t service|socket|automount --state failed|running` : lista todos as unidades de serviço/socket/auto-montável filtrando por estado executando/falhou
- `systemctl cat unidade` : mostra o conteúdo e o caminho absoluto do arquivo de uma unidade

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/systemctl.html
