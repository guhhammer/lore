# choco install

Pasang suatu atau beberapa paket dengan Chocolatey.

## Usage
- `choco install nama_paket1 nama_paket2 ...` : pasang suatu atau beberapa paket yang dipisahkan oleh spasi
- `choco install jalan\menuju\berkas_daftar_paket.config` : pasang kumpulan paket dari suatu berkas konfigurasi
- `choco install jalan\menuju\berkas` : pasang suatu paket dari berkas `.nuspec` atau `.nupkg` secara spesifik
- `choco install nama_paket --version versi` : pasang suatu paket dengan versi spesifik
- `choco install nama_paket --allow-multiple` : izinkan untuk memasang beberapa versi dari paket yang sama
- `choco install nama_paket -y` : lakukan konfirmasi perizinan secara otomatis
- `choco install nama_paket -s url_sumber|alias` : tentukan suatu sumber untuk mendapatkan kumpulan paket
- `choco install nama_paket -u username -p kata_sandi` : sediakan nama pengguna (username) dan kata sandi untuk autentikasi

## Links
- More information: https://docs.chocolatey.org/en-us/choco/commands/install/
