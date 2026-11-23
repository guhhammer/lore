# op

CLI ufficiale per l'app desktop di 1Password.

## Usage
- `op signin` : accedi a un account 1Password
- `op vault list` : elenca tutte le casseforti
- `op item get nome_elemento --format json` : mostra i dettagli di un elemento in formato JSON
- `op item create --category nome_categoria` : crea un nuovo elemento di una categoria nella cassaforte predefinita
- `op read riferimento_segreto` : stampa un segreto referenziato su `stdout`
- `op run -- comando` : passa a un comando i riferimenti ai segreti dalle variabili d'ambiente esportate
- `op run --env-file percorso/del/file_env.env -- comando` : passa a un comando i riferimenti ai segreti da un file di ambiente
- `op inject --in-file percorso/del/file_input --out-file percorso/del/file_output` : leggi i riferimenti ai segreti da un file e salva i segreti in chiaro in un altro file

## Links
- More information: https://developer.1password.com/docs/cli/reference/
