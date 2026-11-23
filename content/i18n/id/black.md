# black

Tulis ulang kode program Python secara otomatis.

## Usage
- `black jalan/menuju/berkas_atau_direktori` : tulis ulang kode dalam suatu berkas atau direktori
- `black -c "kode"` : tulis ulang kode yang dimasukkan sebagai teks string
- `black --check jalan/menuju/berkas_atau_direktori` : tampilkan apakah isi kode suatu berkas atau direktori akan ditulis ulang atau tidak
- `black --diff jalan/menuju/berkas_atau_direktori` : tampilkan daftar perubahan penulisan kode pada berkas atau direktori tanpa menjalankannya (dry-run)
- `black -q jalan/menuju/berkas_atau_direktori` : lakukan penulisan ulang dan hanya keluarkan pesan galat dalam `stderr`
- `black -S jalan/menuju/berkas_atau_direktori` : lakukan penulisan ulang namun jangan menggantikan tanda petik tunggal menuju ganda (pembantu adopsi gaya penulisan bahasa Python berbasis Black, hindari menggunakan ini untuk proyek baru)
