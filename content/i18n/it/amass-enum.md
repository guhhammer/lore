# amass enum

Trova i sottodomini di un dominio.

## Usage
- `amass enum -d nome_dom` : trova (passivamente) i sottodomini di un dominio
- `amass enum -active -d nome_dom -p 80,443,8080` : trova i sottodomini di un dominio e li verifica attivamente tentando di risolvere i sottodomini trovati
- `amass enum -brute -d nome_dom` : esegue una ricerca a forza bruta per i sottodomini
- `amass enum -o file_output -d nome_dom` : salva i risultati in un file di testo
- `amass enum -o file_output -dir percorso/della/directory -d nome_dom` : salva l’output del terminale in un file e altri output dettagliati in una directory
- `amass enum -list` : elenca tutte le fonti dati disponibili
