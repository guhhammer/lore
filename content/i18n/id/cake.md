# cake

Alat baris perintah untuk memproses proyek aplikasi web berbasis framework CakePHP.

## Usage
- `cake` : tampilkan informasi dasar mengenai proyek aplikasi saat ini beserta daftar perintah
- `cake routes` : tampilkan daftar rute aplikasi web yang tersedia
- `cake cache clear_all` : hapus berkas-berkas cache konfigurasi proyek
- `cake schema_cache build --connection koneksi` : bangun berkas cache metadata bagi proyek saat ini
- `cake schema_cache clear` : hapus berkas cache metadata
- `cake schema_cache clear nama_tabel` : hapus suatu tabel cache
- `cake server` : jalankan sebuah peladen web untuk kepentingan pengembangan (secara bawaan mengarah menuju port 8765)
- `cake console` : jalankan sebuah proses REPL (syel interaktif)
