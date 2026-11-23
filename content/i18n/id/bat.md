# bat

Cetak dan gabungkan berkas. Sebuah klon atas program `cat` dengan sintaks berwarna dan integrasi Git.

## Usage
- `bat jalan/menuju/berkas1 jalan/menuju/berkas2 ...` : cetak rapi konten berkas ke `stdout`
- `bat jalan/menuju/berkas1 jalan/menuju/berkas2 ... >jalan/menuju/berkas_tujuan` : gabungkan konten beberapa berkas ke berkas tujuan
- `bat -pp jalan/menuju/berkas` : hapus dekorasi dan matikan fitur tampilan halaman (paging)
- `bat -H 10|5:10|:10|10:|10:+5 jalan/menuju/berkas` : sorot baris tertentu dengan warna latar belakang yang berbeda
- `bat -A jalan/menuju/berkas` : tunjukkan segala karakter yang tak tercetak seperti spasi, tab, atau indikator baris baru
- `bat -n jalan/menuju/berkas` : hapus seluruh informasi dekoratif selain nomor baris pada luaran program
- `bat -l json jalan/menuju/berkas.json` : tampilkan sintaks berwarna terhadap berkas JSON dengan mengatur bahasa sintaks berkas secara eksplisit
- `bat -L` : tampilka semua jenis bahasa sintaks berkas yang didukung

## Links
- More information: https://manned.org/bat
