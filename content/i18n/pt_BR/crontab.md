# crontab

Agenda tarefas cron para serem executadas em um intervalo de tempo para o usuário atual.

## Usage
- `crontab -e` : edita o arquivo crontab para o usuário atual
- `sudo crontab -e -u usuário` : edita o arquivo crontab para um usuário específico
- `crontab caminho/para/arquivo` : substitui o crontab atual pelo conteúdo do arquivo fornecido
- `crontab -l` : exibe uma lista de tarefas cron existentes para o usuário atual
- `crontab -r` : remove todos as tarefas de cron do usuário atual
- `0 10 * * * comando_para_executar` : exemplo de tarefa executada às 10:00 todos os dias (* significa qualquer valor)
- `*/10 * * * * comando_para_executar` : exemplo de entrada do crontab, que executa um comando a cada 10 minutos
- `30 2 * * Fri /caminho/absoluto/para/script.sh` : exemplo de entrada do crontab, que executa um determinado script às 02:30 todas as sextas-feiras

## Links
- More information: https://manned.org/crontab
