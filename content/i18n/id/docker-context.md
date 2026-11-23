# docker context

Pindah antar konteks untuk mengelola beberapa lingkungan pemasangan Docker.

## Usage
- `docker context create nama_konteks --docker "host=tcp://host-jarak-jauh:2375"` : buat suatu konteks baru menggunakan titik akses (endpoint) peladen Docker
- `docker context create nama_konteks` : buat suatu konteks baru berdasarkan informasi nilai variable lingkungan `$DOCKER_HOST` saat ini
- `docker context use nama_konteks` : pindah menuju konteks lainnya
- `docker context ls` : tampilkan daftar nama konteks

## Links
- More information: https://docs.docker.com/reference/cli/docker/context/
