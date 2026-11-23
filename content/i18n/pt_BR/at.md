# at

Ferramenta para o agendamento de comandos. Resultados serão enviados para o e-mail dos usuários.

## Usage
- `systemctl start atd` : inicia o daemon `atd`
- `at now + 5 minutes` : cria comandos interativamente e executa-os em 5 minutos (pressione `<Ctrl d>` quando acabar)
- `at hh:mm` : cria comandos interativamente e executa-os no horário específico
- `echo "comando" | at 1000` : executa um comando da `stdin` às 10:00 da manhã de hoje
- `at -f caminho/para/arquivo 9:30 PM Tue` : executa comandos de um dado arquivo na próxima terça

## Links
- More information: https://manned.org/at
