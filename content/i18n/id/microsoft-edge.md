# microsoft-edge

Aplikasi peramban web (web browser) dari Microsoft yang dibangun berdasarkan peramban Chromium yang dikembangkan oleh Google. Perintah ini tersedia sebagai `msedge` dalam perangkat Windows. Catatan: Anda mungkin dapat menggunakan argumen perintah `chromium` lainnya untuk dapat mengatur jalannya Microsoft Edge.

## Usage
- `microsoft-edge https://example.com|jalan/menuju/berkas.html` : buka suatu URL atau berkas
- `microsoft-edge --inprivate example.com` : buka dalam mode peramban privat (InPrivate)
- `microsoft-edge --new-window example.com` : buka dalam jendela aplikasi baru
- `microsoft-edge --app=https://example.com` : buka dalam mode aplikasi web (tanpa bilah toolbar, URL bar, tombol navigasi, dsb.)
- `microsoft-edge --proxy-server="socks5://hostname:66" example.com` : hubungkan peramban dengan suatu peladen proksi
- `microsoft-edge --user-data-dir=jalan/menuju/direktori` : buka dengan direktori profil pengguna tertentu
- `microsoft-edge --user-data-dir=jalan/menuju/direktori --disable-web-security` : buka dengan menonaktifkan validasi CORS (berguna untuk menguji akses suatu API)
- `microsoft-edge --auto-open-devtools-for-tabs` : selalu buka jendela alat DevTools (pembantu pengembang web) setiap kali membuka tab baru

## Links
- More information: https://microsoft.com/edge
