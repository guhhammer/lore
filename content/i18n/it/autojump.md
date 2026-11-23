# autojump

Salta velocemente tra le directory che visiti più spesso. Alias come `j` o `jc` sono disponibili per una digitazione ancora più veloce.

## Usage
- `source /usr/share/autojump/autojump.bash|fish|zsh` : aggiungi gli alias `autojump` al tuo shell
- `j pattern` : salta in una directory che contiene il pattern specificato
- `jc pattern` : salta in una sotto-directory (figlia) della directory corrente che contiene il pattern specificato
- `jo pattern` : apri una directory che contiene il pattern specificato nel gestore file del sistema operativo
- `j --purge` : rimuovi le directory non esistenti dal database di `autojump`
- `j -s` : mostra le voci nel database di `autojump`

## Links
- More information: https://github.com/wting/autojump#name
