# docker container commit

Buat citra (image) baru atas hasil perubahan isi kontainer.

## Usage
- `docker commit kontainer citra:tag` : buat citra baru dari suatu kontainer secara spesifik
- `docker commit -c "CMD perintah" kontainer citra:tag` : gunakan instruksi Dockerfile `CMD` untuk citra yang sedang dibuat
- `docker commit -c "ENV nama_kunci=nilai" kontainer citra:tag` : gunakan instruksi Dockerfile `ENV` untuk citra yang sedang dibuat
- `docker commit -a "nama_pembuat" kontainer citra:tag` : buat suatu citra dengan menentukan nama pembuat (author) dalam metadata
- `docker commit -m "pesan_komentar" kontainer citra:tag` : buat suatu citra dengan menentukan isi pesan komentar dalam metadata
- `docker commit -p false kontainer citra:tag` : buat suatu citra tanpa menjedakan kontainer saat proses komit berlangsung
- `docker commit --help` : tampilkan bantuan

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/commit/
