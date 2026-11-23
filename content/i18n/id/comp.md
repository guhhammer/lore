# comp

Bandingkan isi antar kedua berkas atau himpunan berkas. Gunakan wildcard (*) untukk membandingkan himpunan berkas.

## Usage
- `comp` : bandingkan isi berkas secara interaktif
- `comp jalan\menuju\berkas1 jalan\menuju\berkas2` : bandingkan isi antara kedua berkas
- `comp jalan\menuju\direktori1\* jalan\menuju\direktori2\*` : bandingkan isi antara kedua himpunan berkas
- `comp /d jalan\menuju\berkas1 jalan\menuju\berkas2` : tampilkan perbedaan dalam format desimal
- `comp /a jalan\menuju\berkas1 jalan\menuju\berkas2` : tampilkan perbedaan dalam format aSCII
- `comp /l jalan\menuju\berkas1 jalan\menuju\berkas2` : tampilkan nomor baris di mana perbedaan antar isi terdeteksi
- `comp /c jalan\menuju\berkas1 jalan\menuju\berkas2` : bandingkan berkas-berkas tanpa menghiraukan penggunaan huruf besar/kecil (case-insensitive)
- `comp /n=5 jalan\menuju\berkas1 jalan\menuju\berkas2` : hanya bandingkan isi 5 baris pertama antara kedua berkas
