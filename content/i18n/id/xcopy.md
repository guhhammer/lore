# xcopy

Salin berkas dan direktori.

## Usage
- `xcopy jalan/menuju/berkas_atau_direktori_sumber jalan/menuju/berkas_atau_direktori_tujuan` : salin berkas atau direktori ke lokasi lain
- `xcopy jalan/menuju/berkas_atau_direktori_sumber jalan/menuju/berkas_atau_direktori_tujuan /p` : lihat daftar berkas yang akan disalin sebelum proses salinan dimulai
- `xcopy jalan/menuju/berkas_atau_direktori_sumber jalan/menuju/berkas_atau_direktori_tujuan /t` : hanya buat salinan struktur direktori saja (tanpa memasukkan berkas apapun ke dalamnya)
- `xcopy jalan/menuju/berkas_atau_direktori_sumber jalan/menuju/berkas_atau_direktori_tujuan /e` : salin termasuk direktori-direktori tanpa isi berkas
- `xcopy jalan/menuju/berkas_atau_direktori_sumber jalan/menuju/berkas_atau_direktori_tujuan /o` : salin berkas dan direktori termasuk informasi hak akses pengguna (ACL) masing-masing
- `xcopy jalan/menuju/berkas_atau_direktori_sumber jalan/menuju/berkas_atau_direktori_tujuan /z` : izinkan proses penyalinan berkas atau direktori untuk berlangsung saat koneksi jaringan komputer terputus
- `xcopy jalan/menuju/berkas_atau_direktori_sumber jalan/menuju/berkas_atau_direktori_tujuan /y` : izinkan `xcopy` untuk tetap mengganti berkas yang sudah ada di lokasi tujuan dengan berkas yang berada di lokasi sumber (override)
- `xcopy /?` : tampilkan bantuan
