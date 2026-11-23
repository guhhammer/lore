# autossh

Jalankan, awasi, dan hubungkan ulang koneksi-koneksi SSH. Menghubungkan koneksi SSH kembali secara otomatis untuk menjaga terowongan penerusan port tetap aktif. Menerima semua argumen standar perintah SSH.

## Usage
- `autossh -M port_untuk_diawasi "perintah_ssh"` : jalankan suatu sesi SSH, hubungkan kembali saat port yang diawasi (Monitoring) gagal mengembalikan data
- `autossh -M port_untuk_diawasi -L port_lokal:localhost:port_jarak_jauh pengguna@hostname` : teruskan suatu port Lokal menuju port jarak jauh, hubungkan kembali bila diperlukan
- `autossh -f -M port_untuk_diawasi -N "perintah_ssh"` : jalankan `autossh` ke dalam proses latar belakang sebelum menjalankan SSH dan jangan (No) membuka sesi shell jarak jauh
- `autossh -f -M 0 -N -o "ServerAliveInterval 10" -o "ServerAliveCountMax 3" "perintah_ssh"` : jalankan dalam latar belakang, daripada mengawasi port apapun, kirimkan paket SSH keep-alive setiap 10 detik untuk mendeteksi kegagalan koneksi
- `autossh -f -M 0 -N -o "ServerAliveInterval 10" -o "ServerAliveCountMax 3" -o ExitOnForwardFailure=yes -L port_lokal:localhost:port_jarak_jauh pengguna@hostname` : jalankan dalam latar belakang, tanpa mengawasi port apapun maupun membuka sesi shell, dan hentikan saat proses penerusan port gagal
- `AUTOSSH_DEBUG=1 AUTOSSH_LOGFILE=jalan/menuju/berkas_log_autossh.log autossh -f -M port_untuk_diawasi -v -E jalan/menuju/berkas_log_ssh.log perintah_ssh` : jalankan dalam latar belakang, dan simpan log awakutu `autossh` dan SSH menuju berkas-berkas tertentu

## Links
- More information: https://manned.org/autossh
