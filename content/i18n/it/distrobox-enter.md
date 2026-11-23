# distrobox-enter

Entra in un container Distrobox. Il comando predefinito eseguito è la tua `$SHELL`, ma puoi specificare shell diverse o interi comandi da eseguire. Se usato dentro uno script, un'applicazione o un servizio, puoi usare la modalità `--headless` per disabilitare il tty e l'interattività.

## Usage
- `distrobox-enter container_name` : entra in un container Distrobox
- `distrobox-enter container_name -- sh -l` : entra in un container Distrobox ed esegui un comando al login
- `distrobox-enter -n container_name -- uptime --pretty` : entra in un container Distrobox senza istanziare un tty

## Links
- More information: https://distrobox.it/usage/distrobox-enter/
