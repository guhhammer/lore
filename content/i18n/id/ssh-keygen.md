# ssh-keygen

Membuat kunci SSH yang digunakan untuk autentikasi, login tanpa kata sandi, dan hal lainnya.

## Usage
- `ssh-keygen` : buat kunci kriptografi secara interaktif
- `ssh-keygen -t ed25519 -a 32 -f ~/.ssh/nama_berkas` : buat kunci kriptografi jenis ed25519 dengan 32 putaran fungsi derivasi kunci (Key Derivation Function) dan simpan kunci ke berkas/fail tertentu
- `ssh-keygen -t rsa -b 4096 -C "komentar|email"` : buat kunci kriptografi jenis RSA 4096-bit dengan email sebagai komentar
- `ssh-keygen -R host_jarak_jauh` : hapus kunci sebuah host dari berkas/fail `known_hosts` (digunakan saat host yang dikenal memiliki kunci kriptografi baru)
- `ssh-keygen -l -E md5 -f ~/.ssh/nama_berkas` : ambil sidik jari (fingerprint) dari sebuah kunci kriptografi dalam format MD5 Hex
- `ssh-keygen -p -f ~/.ssh/nama_berkas` : ubah kata sandi (passphrase) sebuah kunci kriptografi
- `ssh-keygen -p -N "" -m PEM -f ~/.ssh/kunci_privat_OpenSSH` : ubah tipe format kunci (contohnya dari format OPENSSH ke PEM), berkas akan ditimpa di tempat yang sama
- `ssh-keygen -y -f ~/.ssh/kunci_privat_OpenSSH` : ambil kunci publik dari kunci rahasia (privat)

## Links
- More information: https://man.openbsd.org/ssh-keygen
