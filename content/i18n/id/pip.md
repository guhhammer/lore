# pip

Pengelola paket Python. Beberapa subperintah seperti `install` mempunyai dokumentasi terpisah.

## Usage
- `pip install nama_paket` : pasang suatu paket (lihat dokumentasi `pip install` untuk melihat contoh pemasangan tambahan)
- `pip install --user nama_paket` : pasang suatu paket untuk hanya digunakan oleh pengguna saat ini
- `pip install -U nama_paket` : tingkatkan suatu paket ke versi terbaru
- `pip uninstall nama_paket` : copot pemasangan suatu paket
- `pip freeze >requirements.txt` : simpan daftar paket-paket terpasang ke dalam suatu berkas
- `pip show nama_paket` : tampilkan informasi suatu paket yang terpasang
- `pip install -r requirements.txt` : pasang kumpulan paket dari suatu berkas

## Links
- More information: https://pip.pypa.io/en/stable/cli/pip/
