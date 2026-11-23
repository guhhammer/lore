# wmic

Syel interaktif untuk menampilkan informasi terperinci tentang kumpulan proses Windows yang berjalan.

## Usage
- `wmic alias klausa_di_mana klausa_kata_kerja` : tata bahasa dasar perintah
- `wmic process list brief` : tampilkan informasi seluruh proses yang sedang berjalan secara ringkas
- `wmic process list full` : tampilkan informasi seluruh proses yang sedang berjalan secara rinci
- `wmic process get name,processid,parentprocessid` : dapatkan data kolom spesifik seperti nama prosess, nomor induk (ID), dan nomor induk proses induk
- `wmic process where name="example.exe" list full` : tampilkan informasi tentang suatu proses
- `wmic process where processid=pid get name,commandline` : tampilkan data kolom-kolom spesifik terhadap suatu proses
- `wmic process pid delete` : bunuh suatu proses
