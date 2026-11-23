# apptainer cache

Kelola cache Apptainer lokal.

## Usage
- `apptainer cache list` : tampilkan semua image kontainer dalam cache
- `apptainer cache list -v` : tampilkan image kontainer dalam cache dengan informasi detail
- `apptainer cache list -T library|oci|shub|blob|...` : tampilkan hanya jenis cache tertentu
- `apptainer cache clean` : bersihkan seluruh cache
- `apptainer cache clean -T library|oci|shub|blob|...` : bersihkan hanya jenis cache tertentu
- `apptainer cache clean -D days` : bersihkan entri cache yang usianya lebih lama dari jumlah hari tertentu
- `apptainer cache clean -n` : tampilkan pratinjau tentang apa yang akan dibersihkan tanpa menghapus apa pun
- `apptainer cache clean -f` : bersihkan secara paksa tanpa konfirmasi

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_cache.html
