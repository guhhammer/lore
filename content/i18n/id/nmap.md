# nmap

Alat penjelajah jaringan komputer dan pemindai keamanan/port jaringan komputer. Beberapa fitur (seperti pemindaian SYN) hanya aktif jika `nmap` dijalankan menggunakan hak istimewa pengguna root.

## Usage
- `nmap -v1|2|3 ip_atau_hostname` : pindai 1000 port paling umum untuk suatu host jarak jauh dengan menentukan tingkat perincian (verbosity) log pemindaian
- `nmap -T5 -sn 192.168.0.0/24|ip_atau_hostname1,ip_atau_hostname2,...` : jalankan serangan sapuan ping (ping sweep) secara sangat agresif terhadap suatu subnet atau kumpulan host individu
- `sudo nmap -A -iL jalan/menuju/berkas.txt` : nyalakan fitur deteksi sistem operasi, deteksi sistem, pemindaian naskah, dan jejak traceroute atas kumpulan host dari suatu berkas
- `nmap -p port1,port2,... ip_atau_host1,ip_atau_host2,...` : pindai kumpulan host terhadap kumpulan port tertentu (gunakan `-p-` untuk seluruh port dari 1 hingga 65535)
- `nmap -sC -sV -oA top-1000-ports ip_atau_host1,ip_atau_host2,...` : lakukan proses deteksi layanan dan versi terhadap 1000 port paling umum menggunakan naskah NSE bawaan, tuliskan luaran program (`-oA`) ke dalam berkas-berkas baru berawalan nama tertentu
- `nmap --script "default and safe" ip_atau_host1,ip_atau_host2,...` : periksa kumpulan target secara hati-hati menggunakan skrip NSE `default and safe`
- `nmap --script "http-*" ip_atau_host1,ip_atau_host2,... -p 80,443` : pindai kumpulan peladen web yang berjalan dalam port standar 80 dan 443 menggunakan seluruh skrip NSE dengan nama yang berawalkan `http-*`
- `sudo nmap -T0 -D ip_umpan1,ip_umpan2,... --source-port 53 -f --data-length 16 -Pn ip_atau_host` : coba hindari deteksi sistem IDS/IPS dengan memindai secara sangat perlahan (`-T0`), alamat ip sumber umpan (Decoy), paket terfragmentasi, data acak, dan metode lainnya

## Links
- More information: https://nmap.org/book/man.html
