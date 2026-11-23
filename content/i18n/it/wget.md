# wget

Scarica file dal Web. Supporta HTTP, HTTPS, FTP.

## Usage
- `wget https://example.com/abcd` : scarica il contenuto dell'URL in un file (dal nome "abcd" in questo caso)
- `wget -O percorso/del/file https://example.com/abcd` : scarica il contenuto dell'URL in un file (dal nome "efgh" in questo caso)
- `wget -pkw 3 https://example.com/pagina_web.html` : scarica una singola pagina web e tutte le sue risorse (script, immagini, stili, ecc..) aspettando 3 secondi dopo ogni richiesta
- `wget -mnp https://example.com/unqualchepercorso/` : scarica tutti i file elencati nella directory e nelle sue sotto-directory (non scarica gli elementi incorporati nella pagina)
- `wget --limit-rate 300k -t 100 https://example.com/unqualchepercorso/` : limita la velocità di download e il numero di tentativi di connessione
- `wget --user nome_utente --password password https://example.com` : scarica un file da un server HTTP trasmettendo le credenziali tramite Basis Auth (funzione anche con FTP)
- `wget -c https://example.com` : riprende un download incompleto
- `wget -P percorso/della/directory -i lista_di_url.txt` : scarica tutti gli URL contenuti in un file di testo in una directory specificata

## Links
- More information: https://www.gnu.org/software/wget/manual/wget.html
