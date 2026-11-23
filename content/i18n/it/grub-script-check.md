# grub-script-check

Il programma `grub-script-check` prende un file script GRUB e lo controlla per errori di sintassi. Può accettare un percorso come argomento non-opzione. Se non viene fornito, legge da `stdin`.

## Usage
- `grub-script-check path/to/grub_config_file` : verifica un file script specifico per errori di sintassi
- `grub-script-check -v` : mostra ogni riga di input dopo averla letta
- `grub-script-check --help` : mostra l'aiuto
- `grub-script-check --version` : mostra la versione
