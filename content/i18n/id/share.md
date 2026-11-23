# share

Sediakan sumber daya / sistem penyimpanan berkas lokal tersedia untuk dipasang dan dibagikan oleh sistem komputer jarak jauh.

## Usage
- `share` : tampilkan daftar seluruh sistem penyimpanan berkas (filesystem) yang sedang dibagikan
- `share -F nfs -o rw /jalan/menuju/direktori` : bagikan suatu direktori dengan hak akses baca/tulis
- `share -F nfs -o ro /jalan/menuju/direktori` : bagikan suatu direktori dengan hak akses baca saja (read-only)
- `share -F nfs -o rw,root=hostname /jalan/menuju/direktori` : bagikan suatu direktori dengan opsi hak akses tertentu (misalnya, izinkan akses root dari suatu host secara spesifik)
- `echo "share -F nfs -o rw /jalan/menuju/direktori" >>/etc/dfs/dfstab` : buat konfigurasi berbagi persisten dengan menambahkan entri perintah ke dalam `/etc/dfs/dfstab`

## Links
- More information: https://docs.oracle.com/cd/E36784_01/html/E36825/gntjt.html
