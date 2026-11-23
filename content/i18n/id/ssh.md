# ssh

Secure Shell adalah protokol yang digunakan untuk masuk (log on) secara aman ke sistem jarak jauh. Dapat digunakan untuk masuk atau menjalankan perintah pada server jarak jauh.

## Usage
- `ssh nama_pengguna@host_jarak_jauh` : terhubung ke server jarak jauh
- `ssh nama_pengguna@host_jarak_jauh -i lokasi/ke/berkas_kunci` : terhubung ke server jarak jauh dengan identitas (kunci privat) tertentu
- `ssh nama_pengguna@10.0.0.1 -p 2222` : terhubung ke server jarak jauh dengan IP `10.0.0.1` dan menggunakan port tertentu (Catatan: `10.0.0.1` bisa disingkat menjadi `10.1`)
- `ssh nama_pengguna@host_jarak_jauh -t perintah argumen_perintah` : jalankan perintah di server jarak jauh dengan alokasi tty agar dapat berinteraksi dengan perintah tersebut
- `ssh nama_pengguna@host_jarak_jauh -D 1080` : SSH tunneling: Dynamic port forwarding (proksi SOCKS pada `localhost:1080`)
- `ssh nama_pengguna@host_jarak_jauh -L 9999:example.org:80 -N -T` : SSH tunneling: Meneruskan (forward) port tertentu (`localhost:9999` ke `example.org:80`) sekaligus menonaktifkan alokasi pseudo-Tty dan eksekusi perintah jarak jauh (N)
- `ssh nama_pengguna@host_jarak_jauh -J nama_pengguna@host_jump` : SSH Jumping: Terhubung ke server jarak jauh melalui sebuah jumphost (Beberapa lompatan jump dapat ditentukan, dipisahkan dengan koma)
- `<Enter><~><.>` : tutup sesi yang macet (hang)

## Links
- More information: https://man.openbsd.org/ssh
