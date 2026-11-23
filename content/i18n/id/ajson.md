# ajson

Jalankan ekspresi pencarian JSONPath terhadap objek-objek JSON.

## Usage
- `ajson '$..json[?(@.path)]' jalan/menuju/berkas.json` : baca file JSON dan jalankan ekpresi JSONPath untuk mencari data di dalamnya
- `cat jalan/menuju/berkas.json | ajson '$..json[?(@.path)]'` : baca JSON dari `stdin` dan jalankan ekpresi JSONPath untuk mencari data di dalamnya
- `ajson 'avg($..price)' 'https://example.com/api/'` : baca JSON dari sebuah URL dan jalankan ekpresi JSONPath untuk mencari data di dalamnya
- `echo '3' | ajson '2 * pi * $'` : baca file JSON sederhana dan hitung nilai dari ekspresi JSONPath

## Links
- More information: https://github.com/spyzhov/ajson#console-application
