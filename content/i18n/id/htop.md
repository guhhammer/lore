# htop

Tampilkan informasi waktu nyata dinamis tentang proses yang berjalan. Suatu program `top` yang lebih disempurnakan.

## Usage
- `htop` : mulai `htop`
- `htop -u nama_pengguna` : mulai `htop` dan tampilkan proses yang dimiliki oleh pengguna tertentu
- `htop -t` : tampilkan daftar proses beserta hierarki penugasannya dalam bentuk tampilan pohon untuk menunjukkan relasi proses induk beserta anak-anaknya
- `htop -s sort_item` : urutkan proses berdasarkan `sort_item` yang ditentukan (gunakan `htop --sort help` untuk opsi yang tersedia)
- `htop -d 50` : jalankan `htop` dengan jangka waktu pemuatan ulang (refresh) data tertentu, dalam bentuk sepersepuluh detik (yakni 50 = 5 detik)
- `htop --readonly` : matikan seluruh fitur pengubahan terhadap sistem dan proses
- `<F1>|<?>` : lihat perintah interaktif saat menjalankan `htop`
- `<Tab>` : alihkan tampilan menuju tab lain

## Links
- More information: https://manned.org/htop
