# nixos-container

Jalankan kumpulan kontainer NixOS menggunakan sistem kontainer Linux.

## Usage
- `sudo nixos-container list` : tampilkan daftar kontainer yang sedang berjalan
- `sudo nixos-container create nama_kontainer --config-file jalan/menuju/berkas_konfigurasi_nix` : buat suatu kontainer NixOS menggunakan berkas konfigurasi tertentu
- `sudo nixos-container start|stop|terminate|destroy|status nama_kontainer` : jalankan, hentikan, matikan, hancurkan, atau tampilkan kondisi suatu kontainer
- `sudo nixos-container run nama_kontainer -- perintah argumen_perintah` : jalankan sebuah perintah dalam suatu kontainer berjalan
- `sudo $EDITOR /var/lib/container/nama_kontainer/etc/nixos/configuration.nix && sudo nixos-container update nama_kontainer` : perbarui konfigurasi suatu kontainer
- `sudo nixos-container root-login nama_kontainer` : masuk ke dalam sesi shell interaktif baru dalam suatu kontainer berjalan

## Links
- More information: https://nixos.org/manual/nixos/stable/#ch-containers
