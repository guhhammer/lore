# clamdscan

Sebuah program baris perintah untuk memindai virus komputer.

## Usage
- `clamdscan jalan/menuju/berkas_atau_direktori` : pindai kerentanan suatu berkas atau direktori tertentu
- `perintah | clamdscan -` : pindai data dari input `stdin`
- `clamdscan --infected` : pindai direktori saat ini dan hanya tampilkan berkas yang terinfeksi
- `clamdscan --log jalan/menuju/berkas_log` : simpan hasil laporan pemindaian kepada sebuah berkas log
- `clamdscan --move jalan/menuju/direktori_karantina` : pindahkan berkas-berkas yang terinfeksi kepada suatu direktori
- `clamdscan --remove` : hapus berkas-berkas yang terinfeksi
- `clamdscan --multiscan` : gunakan lebih dari satu thread untuk memindai sebuah direktori
- `clamdscan --fdpass` : pindai berkas dengan memberikan deskriptor kepada daemon daripada mengoper isi mentah berkas tersebut seperti biasa

## Links
- More information: https://docs.clamav.net/manual/Usage/Scanning.html#clamdscan
