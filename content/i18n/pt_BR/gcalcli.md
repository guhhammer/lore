# gcalcli

Ferramenta de linha de comando para interagir com o Google Agenda. Solicita autorização da API do Google na primeira inicialização.

## Usage
- `gcalcli agenda` : lista seus eventos para todos os seus calendários nos próximos 7 dias
- `gcalcli agenda mm/dd mm/dd` : mostra eventos começando em ou entre datas específicas (também recebe datas relativas, por exemplo, "amanhã")
- `gcalcli --calendar nome_do_calendário agenda` : lista eventos de um calendário específico
- `gcalcli calw` : exibe um calendário ASCII de eventos por semana
- `gcalcli calm` : exibe um calendário ASCII de eventos para um mês
- `gcalcli --calendar nome_do_calendário quick "mm/dd HH:MM nome_do_evento"` : adiciona um evento rapidamente ao seu calendário
- `gcalcli --calendar "nome_do_calendário" add` : adiciona um evento ao calendário. Dispara prompt interativo

## Links
- More information: https://github.com/insanum/gcalcli
