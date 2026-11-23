# aspell

Mesin pengecek ejaan secara interaktif. Informasi lebih lanjut: <http://aspell.net/man-html/index.html>.

## Usage
- `aspell check jalan/menuju/berkas` : lakukan pengecekan ejaan terhadap suatu berkas
- `cat jalan/menuju/berkas | aspell list` : tampilkan daftar kata dalam `stdin` yang dicurigai memiliki kesalahan ejaan
- `aspell dicts` : tampilkan daftar kamus dan bahasa yang didukung
- `aspell --lang cs` : jalankan `aspell` dengan bahasa teks yang berbeda (menggunakan format kode bahasa ISO 639)
- `cat jalan/menuju/berkas | aspell --personal daftar-kata-pribadi.pws list` : tampilkan daftar kata dalam `stdin` yang dicurigai memiliki kesalahan ejaan dan abaikan kata yang berasal dari daftar kata pribadi (personal word list)
