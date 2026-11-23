# winget

Manajer Paket Antarmuka Baris Perintah Windows.

## Usage
- `winget add paket` : pasang suatu paket
- `winget rm paket` : hapus paket yang terpasang sebelumnya (Catatan: subperintah `uninstall` juga dapat digantikan dengan `remove`)
- `winget show paket` : tampilkan informasi tentang paket
- `winget search paket` : cari paket
- `winget upgrade -r` : perbarui seluruh paket menuju versi terkini
- `winget ls -s winget` : tampilkan paket terpasang yang dapat dikelola oleh `winget`
- `winget import|export --import-file|--output jalan\menuju\berkas` : impor atau ekspor daftar paket terpasang ke dalam suatu file
- `winget validate jalan\menuju\manifes` : lakukan uji validasi manifes pemaketan winget sebelum mengirimkan rencana perubahan (Pull Request) menuju repositori winget-pkgs

## Links
- More information: https://learn.microsoft.com/windows/package-manager/winget
