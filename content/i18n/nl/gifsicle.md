# gifsicle

Maak, bewerk, manipuleer en verkrijg informatie over GIF-bestanden.

## Usage
- `gifsicle pad/naar/invoer.gif -O3 -o pad/naar/uitvoer.gif` : optimaliseer een GIF als een nieuw bestand
- `gifsicle -b pad/naar/invoer.gif -U` : gebruik batchmodus (wijzig elk opgegeven bestand direct) en de-optimaliseer een GIF
- `gifsicle pad/naar/invoer.gif '#0' >pad/naar/eerste_frame.gif` : extraheer een frame uit een GIF
- `gifsicle *.gif -d 10 -l >pad/naar/uitvoer.gif` : maak een GIF-animatie van geselecteerde GIFs
- `gifsicle -b pad/naar/invoer.gif -O3 --lossy=100 -k 16 -f` : verklein de bestandsgrootte met behulp van lossy-compressie
- `gifsicle -b pad/naar/invoer.gif --delete '#0-9' '#20-'` : verwijder de eerste 10 frames en alle frames na frame 20 uit een GIF
- `gifsicle -b --crop start_x,start_y+rechthoek_breedtexrechthoek_hoogte --scale 0.25 --flip-horizontal --rotate-90|180|270 pad/naar/invoer.gif` : wijzig alle frames door ze bij te snijden tot een rechthoek, hun schaal te wijzigen, ze te spiegelen en te roteren

## Links
- More information: https://www.lcdf.org/gifsicle/
