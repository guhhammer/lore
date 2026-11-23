# expand

Vervang tabs met spaties.

## Usage
- `expand pad/naar/bestand` : vervang tabs in ieder bestand met spaties en schrijf het naar `stdout`
- `expand` : vervang tabs met spaties, lezend vanaf `stdin`
- `expand -i pad/naar/bestand` : vervang geen tabs na niet-lege tekens
- `expand -t nummer pad/naar/bestand` : laat tabs een bepaald aantal tekens uit elkaar staan, niet 8
- `expand -t 1,4,6` : gebruik een door komma's gescheiden lijst met expliciete tabposities
