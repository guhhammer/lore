# exiftool

Leggi e scrivi metadati nei file.

## Usage
- `exiftool -All= file1 file2 ...` : rimuovi tutti i metadati EXIF dai file specificati
- `exiftool "-AllDates+=0:0:0 1:0:0" percorso/della/directory` : muovi avanti di 1 ora la data in cui sono state scattate tutte le foto contenute in una directory
- `exiftool "-AllDates-=0:0:1 2:0:0" -ext jpg` : muovi indietro di 1 giorno e 2 ore la data in cui sono state scattate tutte le immagini JPEG
- `exiftool -DateTimeOriginal-=1.5 -overwrite_original` : cambia solo il campo `DateTimeOriginal` sottraendo 1.5 ore e non tenere file di backup
- `exiftool '-filename<DateTimeOriginal' -d %Y-%m-%d_%H-%M-%S%%lc.%%e percorso/della/directory -r -ext jpg` : rinomina ricorsivamente tutti i file JPEG in una directory in base al campo `DateTimeOriginal`

## Links
- More information: https://exiftool.org/exiftool_pod.html
