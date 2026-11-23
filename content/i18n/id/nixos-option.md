# nixos-option

Lakukan inspeksi terhadap konfigurasi sistem NixOS.

## Usage
- `nixos-option kata_kunci_pengaturan` : tampilkan seluruh subkunci dari suatu kata kunci pengaturan (option key)
- `nixos-option boot.kernelModules` : tampilkan daftar modul kernel yang dimuat saat proses boot
- `nixos-option users.users.nama_pengguna.openssh.authorizedKeys.keyFiles|keys` : tampilkan daftar kunci OpenSSH yang diizinkan untuk mewakili suatu pengguna
- `nixos-option nix.buildMachines` : tampilkan daftar seluruh komputer pembantu pembangun paket piranti lunak (remote builder)
- `NIXOS_CONFIG=jalan/menuju/konfigurasi.nix nixos-option kata_kunci_pengaturan` : tampilkan daftar seluruh subkunci dari kata kunci (option key) dalam berkas konfigurasi sistem NixOS
- `nixos-option -r users.users.pengguna` : tampilkan seluruh kunci beserta nilai konfigurasi bagi suatu pengguna secara rekursif

## Links
- More information: https://nixos.org/manual/nixos/stable/index.html#sec-modularity
