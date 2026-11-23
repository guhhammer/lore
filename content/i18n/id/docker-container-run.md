# docker container run

Jalankan suatu perintah dalam suatu kontainer Docker baru.

## Usage
- `docker run citra:tag perintah` : jalankan perintah dalam suatu kontainer baru dari suatu citra dengan label/tag tertentu
- `docker run -d citra perintah` : jalankan perintah dalam suatu kontainer baru secara latar belakang dan tampilkan ID kontainer baru yang dibuat
- `docker run --rm -it citra perintah` : jalankan perintah dalam sebuah kontainer sekali pakai dalam mode interaktif dan pseudo-TTY
- `docker run -e 'variabel=nilai' -e variabel citra perintah` : jalankan perintah dalam suatu kontainer dengan meneruskan variabel lingkungan
- `docker run -v /jalan/menuju/berkas_atau_direktori_penyimpanan_induk:/jalan/menuju/berkas_atau_direktori_dalam_kontainer citra perintah` : jalankan perintah dalam suatu kontainer baru dengan memadankan volume penyimpanan
- `docker run -p port_perangkat_induk:port_dalam_kontainer citra perintah` : jalankan perintah dalam suatu kontainer baru dengan daftar port untuk diteruskan/dipublikasikan
- `docker run --entrypoint perintah citra` : jalankan perintah dalam suatu kontainer baru dengan menggantikan perintah awal (entrypoint) dalam citra yang dipilih
- `docker run --network network citra` : jalankan perintah dalam suatu kontainer baru dan hubungkan ke dalam suatu jaringan

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/run/
