# aspell

Correttore ortografico interattivo. Maggiori informazioni: <http://aspell.net/man-html/index.html>.

## Usage
- `aspell check percorso/del/file` : controllare l'ortografia di un singolo file
- `cat percorso/del/file | aspell list` : elencare le parole errate da `stdin`
- `aspell dicts` : mostrare i dizionari linguistici disponibili
- `aspell --lang cs` : eseguire `aspell` con una lingua diversa (accetta il codice linguistico ISO 639 a due lettere)
- `cat percorso/del/file | aspell --personal lista_parole_personali.pws list` : elencare le parole errate da `stdin` e ignorare le parole presenti nella lista personale
