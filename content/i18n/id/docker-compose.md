# docker compose

Jalankan dan kelola aplikasi Docker dengan beberapa kontainer.

## Usage
- `docker compose ps` : tampilkan semua kontainer yang sedang berjalan
- `docker compose up -d` : buat dan nyalakan semua kontainer di latar belakang menggunakan file docker-compose.yml dari direktori saat ini
- `docker compose up --build` : nyalakan semua kontainer, dan bangun ulang jika diperlukan
- `docker compose -p nama_proyek -f jalan/menuju/berkas up` : nyalakan semua kontainer dengan menentukan nama proyek dan menggunakan file compose alternatif
- `docker compose stop` : hentikan semua kontainer yang sedang berjalan
- `docker compose down --rmi all -v` : hentikan dan menghapus semua kontainer, jaringan, image, dan volume
- `docker compose logs -f` : ikuti log untuk semua kontainer
- `docker compose logs -f container_name` : ikuti log untuk kontainer tertentu

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/
