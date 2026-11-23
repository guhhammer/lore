# nixos-rebuild

Lakukan konfigurasi ulang bagi suatu mesin NixOS.

## Usage
- `sudo nixos-rebuild switch` : bangun dan pindah menuju sistem NixOS yang dibangun berdasarkan konfigurasi terbaru, sebagai pilihan sistem bawaan terbaru dalam proses boot
- `sudo nixos-rebuild switch -p nama` : bangun dan pindah sistem dengan nama baru yang ditentukan
- `sudo nixos-rebuild switch --upgrade` : bangun dan pindah sistem dengan memutakhirkan seluruh paket piranti lunak di dalamnya
- `sudo nixos-rebuild switch --rollback` : bangun dan pindah sistem dengan mengembalikan pemasangan seluruh paket piranti lunak ke dalam kondisi pra-mutakhir
- `sudo nixos-rebuild boot` : bangun sistem berdasarkan konfigurasi terbaru dan simpan sebagai bawaan baru proses boot tanpa memindahkan sesi saat ini kepadanya
- `sudo nixos-rebuild test` : bangun dan pindah ke sistem baru dalam mode uji coba (tanpa menyimpannya sebagai entri sistem baru dalam proses boot)
- `sudo nixos-rebuild build-vm` : bangun dengan konfigurasi terbaru dan buka sistem terbaru dalam mesin virtual
- `nixos-rebuild list-generations` : tampilkan daftar generasi sistem yang tersedia, serupa dengan daftar menu pada boot loader

## Links
- More information: https://nixos.org/nixos/manual/#sec-changing-config
