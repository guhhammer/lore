# chromium

Aplikasi peramban web (web browser) bersumber terbuka yang terutama dikembangkan dan dikelola oleh Google. Catatan: Anda mungkin perlu menggantikan perintah `chromium` dengan peramban tujuan Anda, seperti `brave`, `google-chrome`, `opera`, atau `vivaldi`.

## Usage
- `chromium https://example.com|jalan/menuju/berkas.html` : buka suatu URL atau berkas
- `chromium --incognito example.com` : buka dalam mode peramban privat (incognito)
- `chromium --new-window example.com` : buka dalam jendela aplikasi baru
- `chromium --app=https://example.com` : buka dalam mode aplikasi web (tanpa bilah toolbar, URL bar, tombol navigasi, dsb.)
- `chromium --proxy-server="socks5://hostname:66" example.com` : hubungkan peramban dengan suatu peladen proksi
- `chromium --user-data-dir=jalan/menuju/direktori` : buka dengan direktori profil pengguna tertentu
- `chromium --user-data-dir=jalan/menuju/direktori --disable-web-security` : buka dengan menonaktifkan validasi CORS (berguna untuk menguji akses suatu API)
- `chromium --auto-open-devtools-for-tabs` : selalu buka jendela alat DevTools (pembantu pengembang web) setiap kali membuka tab baru

## Links
- More information: https://www.chromium.org/developers/how-tos/run-chromium-with-flags/
