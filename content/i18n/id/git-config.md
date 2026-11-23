# git config

Ubah pengaturan Git untuk repositori-repositori tertentu. Konfigurasi ini dapat diatur hanya untuk repositori saat ini (lokal/local) atau untuk pengguna sistem operasi saat ini (global).

## Usage
- `git config --global user.name|user.email "Nama Anda|surel@example.com"` : tentukan nama atau alamat surel Anda secara global (informasi ini diperlukan untuk melakukan proses komit, dan akan dicantumkan dalam seluruh komit Anda)
- `git config --local|global|system -l --show-origin` : tampilkan daftar pengaturan Git untuk lingkungan lokal, global, atau sistem operasi, dan tampilkan lokasi berkas tersebut
- `git config --global alias.unstage "reset HEAD --"` : tentukan nilai global atas suatu entri konfigurasi (dalam hal ini merupakan alias)
- `git config alias.unstage` : dapatkan nilai atas entri konfigurasi saat ini
- `git unstage` : gunakan suatu perintah alias
- `git config --global --unset alias.unstage` : hapus atau kembalikan nilai dari entri konfigurasi tersebut menuju nilai default (bila ada)
- `git config -e` : sunting konfigurasi Git pada repositori saat ini dengan aplikasi pengolah teks default
- `git config --global -e` : sunting konfigurasi Git pada pengguna saat ini (dalam berkas `~/.gitconfig` atau `$XDG_CONFIG_HOME/git/config` bila ada) dengan aplikasi pengolah teks default

## Links
- More information: https://git-scm.com/docs/git-config
