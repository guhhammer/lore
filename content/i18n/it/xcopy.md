# xcopy

Copia di file e directory.

## Usage
- `xcopy percorso\del\file_o_directory percorso\della\destinazione` : copia il/i file nella destinazione specificata
- `xcopy percorso\del\file_o_directory percorso\della\destinazione /p` : elenca i file da copiare prima di copiarli verso la destinazione
- `xcopy percorso\del\file_o_directory percorso\della\destinazione /t` : copia solo la struttura della directory senza i file
- `xcopy percorso\del\file_o_directory percorso\della\destinazione /e` : copia le directory vuote
- `xcopy percorso\del\file_o_directory percorso\della\destinazione /o` : mantieni le politiche di accesso della sorgente (ACL) nella directory di destinazione
- `xcopy percorso\del\file_o_directory percorso\della\destinazione /z` : continua l'operazione dopo l'interruzione della connessione di rete
- `xcopy percorso\del\file_o_directory percorso\della\destinazione /y` : sovrascrivi automaticamente i file di destinazione già esistenti
- `xcopy /?` : mostra l'aiuto
