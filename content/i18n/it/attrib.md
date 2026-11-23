# attrib

Mostra o cambia gli attributi di file e cartelle.

## Usage
- `attrib` : mostra tutti gli attributi dei file in una cartella
- `attrib percorso\della\cartella` : mostra tutti gli attributi dei file in una cartella specifica
- `attrib /d` : mostra tutti gli attributi dei file e delle cartelle nella cartella corrente
- `attrib /s` : mostra tutti gli attributi dei file in una cartella e le sue sotto-cartelle
- `attrib +r|a|s|h|i percorso\del\file_o_cartella1 percorso\del\file_o_cartella2 ...` : aggiungi l'attributo `read-only` o `archive` o `system` o `hidden` o `not content indexed` a file o cartelle
- `attrib -r|a|s|h|i percorso\del\file_o_cartella1 percorso\del\file_o_cartella2 ...` : rimuovi un attributo specifico da un file o una cartella
