# csvsql

Hasilkan pernyataan SQL untuk sebuah file CSV atau jalankan pernyataan tersebut secara langsung pada sebuah basis data (database). Bagian dari csvkit.

## Usage
- `csvsql jalan/menuju/data.csv` : hasilkan sebuah pernyataan SQL CREATE TABLE untuk sebuah file CSV
- `csvsql --insert --db "mysql://pengguna:kata_sandi@host/basis_data" jalan/menuju/data.csv` : impor sebuah file CSV ke dalam sebuah basis data SQL
- `csvsql --query "select * from 'data'" jalan/menuju/data.csv` : jalankan sebuah kueri (query) SQL pada sebuah file CSV

## Links
- More information: https://csvkit.readthedocs.io/en/latest/scripts/csvsql.html
