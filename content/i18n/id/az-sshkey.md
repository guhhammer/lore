# az sshkey

Kelola kumpulan kunci publik SSH dengan instansi mesin virtual (VM). Bagian dari `azure-cli` (juga dikenal sebagai `az`).

## Usage
- `az sshkey create --name nama -g grup_sumber_daya` : buat suatu kunci SSH baru
- `az sshkey create --name nama -g grup_sumber_daya --public-key "@jalan/menuju/kunci.pub"` : unggah suatu kunci publik SSH
- `az sshkey list` : tampilkan daftar kunci publik SSH yang terdaftar
- `az sshkey show --name nama -g grup_sumber_daya` : tampilkan informasi mengenai suatu kunci publik SSH terdaftar

## Links
- More information: https://learn.microsoft.com/cli/azure/sshkey
