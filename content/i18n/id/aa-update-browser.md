# aa-update-browser

Perbarui profil browser AppArmor agar menggunakan abstraksi yang didukung. Bagian dari paket AppArmor.

## Usage
- `sudo aa-update-browser -l` : tampilkan daftar profil abstraksi browser yang tersedia
- `sudo aa-update-browser -d path/ke/profil` : tampilkan perubahan apa yang akan dibuat pada sebuah profil tanpa menerapkannya (dry-run)
- `sudo aa-update-browser -u abstraction1,abstraction2,... path/ke/profil` : perbarui profil dengan abstraksi tertentu
- `sudo aa-update-browser -u '' path/ke/profil` : hapus semua abstraksi dari sebuah profil
- `aa-update-browser -h` : tampilkan bantuan

## Links
- More information: https://manned.org/aa-update-browser
