# sshuttle

Server proksi transparan yang meneruskan lalu lintas (traffic) melalui koneksi SSH. Tidak memerlukan root atau pengaturan khusus di server SSH jarak jauh, namun akses root di mesin lokal akan diminta.

## Usage
- `sshuttle -r nama_pengguna@server_ssh 0.0.0.0/0` : teruskan semua lalu lintas TCP IPv4 melalui server SSH jarak jauh
- `sshuttle --dns -r nama_pengguna@server_ssh 0.0.0.0/0` : teruskan juga semua lalu lintas DNS ke DNS resolver default milik server
- `sshuttle -r nama_pengguna@server_ssh 0.0.0.0/0 -x 192.168.0.1/24` : teruskan semua lalu lintas kecuali yang ditujukan untuk subnet tertentu
- `sshuttle --method tproxy -r nama_pengguna@server_ssh 0.0.0.0/0 ::/0 -x alamat_ip_lokal_anda -x alamat_ip_server_ssh` : gunakan metode tproxy untuk meneruskan semua lalu lintas IPv4 dan IPv6

## Links
- More information: https://manned.org/sshuttle
