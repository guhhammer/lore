# docker login

Masuk menuju suatu registry Docker menggunakan akun terdaftar.

## Usage
- `docker login` : masuk ke dalam suatu registry secara interaktif
- `docker login -u nama_pengguna` : masuk ke dalam suatu registry menggunakan nama pengguna tertentu (kata sandi akan kemudian ditanyakan kepada pengguna)
- `docker login -u nama_pengguna -p kata_sandi peladen` : masuk ke dalam suatu registry menggunakan nama dan kata sandi pengguna
- `echo "kata_sandi" | docker login -u nama_pengguna --password-stdin` : masuk ke dalam suatu registry menggunakan kata sandi dari `stdin`

## Links
- More information: https://docs.docker.com/reference/cli/docker/login/
