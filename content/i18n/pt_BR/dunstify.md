# dunstify

Uma ferramenta de notificação que é uma extensão do `notify-send`, mas com mais funcionalidades baseadas em `dunst`. Aceita todas as opções do `notify-send`.

## Usage
- `dunstify "Título" "Mensagem"` : mostra uma notificação com um dado título e mensagem
- `dunstify "Título" "Mensagem" -u low|normal|critical` : mostra uma notificação com uma urgência específica
- `dunstify "Título" "Mensagem" -r 123` : especifica um ID para a mensagem (sobrescreve qualquer mensagem anterior com o mesmo ID)
- `dunstify -?` : mostra ajuda

## Links
- More information: https://dunst-project.org/documentation/dunstify/
