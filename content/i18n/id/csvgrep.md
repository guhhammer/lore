# csvgrep

Filter baris-baris CSV menggunakan pencocokan string dan pola (pattern matching). Bagian dari csvkit.

## Usage
- `csvgrep -c 1 -m string_to_match data.csv` : temukan baris-baris yang memiliki string tertentu pada kolom 1
- `csvgrep -c 3,4 -r regex data.csv` : temukan baris-baris di mana isi kolom 3 atau 4 cocok dengan ekspresi reguler (`regex`) tertentu
- `csvgrep -i -c nama -m "John Doe" data.csv` : temukan baris-baris di mana kolom "nama" TIDAK memuat string "John Doe"

## Links
- More information: https://csvkit.readthedocs.io/en/latest/scripts/csvgrep.html
