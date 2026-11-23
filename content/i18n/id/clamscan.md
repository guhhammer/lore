# clamscan

Sebuah program baris perintah untuk memindai virus komputer.

## Usage
- `clamscan jalan/menuju/berkas` : pindai kerentanan sebuah berkas
- `clamscan -r jalan/menuju/direktori` : pindai seluruh berkas dalam sebuah direktori secara rekursif
- `perintah | clamscan -` : pindai data dari input `stdin`
- `clamscan -d jalan/menuju/berkas_atau_direktori_basis_data` : gunakan basis data (database) definisi virus yang terkandung dalam sebuah berkas atau direktori
- `clamscan -i` : pindai direktori saat ini dan hanya tampilkan berkas yang terinfeksi
- `clamscan -l jalan/menuju/berkas_log` : simpan hasil laporan pemindaian kepada sebuah berkas log
- `clamscan --move jalan/menuju/direktori_karantina` : pindahkan berkas-berkas yang terinfeksi kepada suatu direktori
- `clamscan --remove yes` : hapus berkas-berkas yang terinfeksi

## Links
- More information: https://docs.clamav.net/manual/Usage/Scanning.html#clamscan
