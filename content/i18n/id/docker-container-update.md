# docker container update

Mutakhirkan konfigurasi sistem dalam kumpulan kontainer Docker. Catatan: Perintah ini tidak mendukung kumpulan kontainer Windows.

## Usage
- `docker update --restart always|no|on-failure|unless-stopped nama_kontainer` : perbarui kebijakan pemuatan ulang (restart policy) saat suatu kontainer berhenti
- `docker update --restart on-failure:3 nama_kontainer` : perbarui kebijakan untuk mencoba memuat ulang hingga tiga kali kesempatan untuk suatu kontainer yang berhenti dengan kode exit non-nihil
- `docker update --cpus jumlah_cpu nama_kontainer` : perbarui jumlah CPU yang dialokasikan untuk suatu kontainer
- `docker update -m batasM nama_kontainer` : perbarui batas memori dalam satuan Megabita untuk suatu kontainer
- `docker update --pids-limit batas nama_kontainer` : perbarui batas maksimum nomor induk proses (Process ID) dalam suatu kontainer (gunakan `-1` untuk tak terbatas)
- `docker update --memory-swap batasM nama_kontainer` : perbarui jumlah memori swap dalam satuan Megabita untuk suatu kontainer (gunakan `-1` untuk tak terbatas)

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/update/
