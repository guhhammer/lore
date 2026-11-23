# docker container ls

Tampilkan daftar kontainer Docker.

## Usage
- `docker ps` : tampilkan kontainer Docker yang sedang berjalan saat ini
- `docker ps -a` : tampilkan semua kontainer Docker (yang berjalan dan yang berhenti)
- `docker ps -l` : tampilkan kontainer yang dibuat terakhir (termasuk semua status)
- `docker ps -f "name=nama"` : pilah kontainer yang mengandung substring dalam namanya
- `docker ps -f "ancestor=image:tag"` : pilah kontainer yang memiliki gambar yang sama sebagai leluhur
- `docker ps -f "exited=kode" -a` : pilah kontainer berdasarkan kode status keluar (exit status code)
- `docker ps -f "status=status"` : pilah kontainer berdasarkan status (created, running, removing, paused, exited, dan dead)
- `docker ps -f "volume=jalan/menuju/direktori" --format "table \{\{.ID\}\}\t\{\{.Image\}\}\t\{\{.Names\}\}\t\{\{.Mounts\}\}"` : pilah kontainer yang mengaitkan suatu volume tertentu atau memiliki volume yang terpasang pada jalur tertentu

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/ls/
