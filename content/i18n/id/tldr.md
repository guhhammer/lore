# tldr

Tampilkan laman bantuan sederhana untuk alat baris perintah (command-line) dari proyek dokumentasi tldr-pages. Catatan: Opsi `--language` dan `--list` sering diimplementasikan oleh program-program klien meskipun tak diwajibkan menurut spesifikasi teknis.

## Usage
- `tldr perintah` : tampilkan laman bantuan sederhana untuk suatu perintah (catatan: beginilah cara Anda sampai di sini!)
- `tldr perintah subperintah` : tampilkan laman bantuan sederhana untuk suatu subperintah
- `tldr -L kode_bahasa perintah` : tampilkan laman bantuan untuk suatu perintah dalam suatu bahasa (jika tersedia, selainnya dalam bahasa Inggris)
- `tldr -p android|cisco-ios|common|dos|freebsd|linux|netbsd|openbsd|osx|sunos|windows perintah` : tampilkan laman bantuan untuk suatu perintah pada platform tujuan
- `tldr -u` : mutakhirkan data cache lokal untuk laman-laman bantuan
- `tldr -l` : tampilkan daftar seluruh laman bantuan untuk perintah-perintah yang tersedia pada platform saat ini dan `common` (lintas platform)
- `tldr -l | fzf --preview "tldr {1} --color=always" --preview-window=right,70% | xargs tldr` : lihat isi laman tldr dalam jendela terminal (`fzf` harus tersedia terlebih dahulu)
- `tldr -l | shuf -n 1 | xargs tldr` : tampilkan suatu laman bantuan untuk perintah yang dipilih secara acak
