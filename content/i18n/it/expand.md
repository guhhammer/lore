# expand

Converti caratteri tab in spazi.

## Usage
- `expand percorso/del/file` : converti tab in un file in spazi, scrivendo su `stdout`
- `expand` : converti i tab in spazi, leggendo da `stdin`
- `expand -i percorso/del/file` : non convertire i tab dopo caratteri di spaziatura
- `expand -t numero_spazi percorso/del/file` : sostituisci i tab con un determinato numero di spazi, non 8 (default)
- `expand -t 1,4,6` : utilizza una lista separata da virgole di posizioni esplicite di tab
