# csvformat

Ubah sebuah file CSV ke dalam format keluaran (output) kustom. Bagian dari csvkit.

## Usage
- `csvformat -T data.csv` : ubah menjadi berkas yang dipisahkan oleh tab (TSV)
- `csvformat -D "karakter_pembatas" data.csv` : ubah pembatas (delimiter) menjadi karakter kustom
- `csvformat -M "\r\n" data.csv` : ubah akhiran baris menjadi carriage return (^M) + line feed
- `csvformat -U 0 data.csv` : minimalkan penggunaan karakter tanda kutip
- `csvformat -U 1 data.csv` : maksimalkan penggunaan karakter tanda kutip

## Links
- More information: https://csvkit.readthedocs.io/en/latest/scripts/csvformat.html
