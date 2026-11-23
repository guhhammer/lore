# docker image ls

Tampilkan daftar citra Docker.

## Usage
- `docker images` : tampilkan daftar seluruh citra Docker yang terpasang
- `docker images -a` : tampilkan daftar seluruh citra Docker termasuk citra-citra perantara (intermediates)
- `docker images -q` : tampilkan daftar dalam mode sunyi (hanya tampilkan nomor induk setiap citra)
- `docker images -f dangling=true` : tampilkan daftar seluruh citra Docker yang tidak dipakai dalam kontainer apapun
- `docker images "*nama*"` : tampilkan daftar kumpulan citra yang memiliki nama dengan pola nama tertentu
- `docker images --format "\{\{.ID\}\}\t\{\{.Size\}\}\t\{\{.Repository\}\}:\{\{.Tag\}\}" | sort -k 2 -h` : urutkan daftar citra berdasarkan ukurannya

## Links
- More information: https://docs.docker.com/reference/cli/docker/image/ls/
