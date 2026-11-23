# distrobox-enter

Entrar em um contêiner Distrobox. O comando padrão executado é o seu `$SHELL`, mas você pode especificar shells diferentes ou comandos completos para serem executados. Se usado dentro de um script, um aplicativo ou um serviço, você pode usar o modo `--headless` para desabilitar o tty e a interatividade.

## Usage
- `distrobox-enter nome_do_contêiner` : entra em um contêiner Distrobox
- `distrobox-enter nome_do_contêiner -- sh -l` : entra em um contêiner Distrobox e executa um comando no login
- `distrobox-enter -n nome_do_contêiner -- uptime -p` : entra em um contêiner Distrobox sem instanciar um tty

## Links
- More information: https://distrobox.it/usage/distrobox-enter/
