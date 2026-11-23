# docker container exec

Jalankan sebuah perintah dalam suatu kontainer Docker yang sedang berjalan.

## Usage
- `docker exec -it nama_kontainer /bin/bash` : masuk ke dalam sebuah sesi shell interaktif dalam suatu kontainer yang sedang berjalan
- `docker exec -d nama_kontainer perintah` : jalankan sebuah perintah dalam latar belakang (mode lepas / detached) dalam suatu kontainer berjalan
- `docker exec -it -w jalan/menuju/direktori nama_kontainer perintah` : tentukan alamat direktori kerja yang menjadi pangkal untuk proses eksekusi sebuah perintah
- `docker exec -i -d nama_kontainer perintah` : jalankan sebuah perintah dalam latar belakang dalam kontainer berjalan namun tetap buka saluran `stdin`
- `docker exec -it -e nama_variabel=nilai nama_kontainer /bin/bash` : tentukan suatu variabel lingkungan untuk suatu sesi shell Bash
- `docker exec -u pengguna nama_kontainer perintah` : jalankan suatu perintah atas nama suatu pengguna dalam kontainer

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/exec/
