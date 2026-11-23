# brew

Homebrew - suatu manajer paket bagi macOS dan Linux. Beberapa subperintah seperti `install` mempunyai dokumentasi terpisah.

## Usage
- `brew install formula|cask` : pasang versi terkini oleh suatu formula atau cask
- `brew list` : tampilkan daftar formula dan cask yang terpasang
- `brew upgrade formula|cask` : mutakhirkan suatu formula atau cask (jika nama tidak disediakan, semua formula dan cask terpasang akan dimutakhirkan)
- `brew update` : dapatkan program Homebrew versi terkini dan semua formula dan cask yang tersedia dari repositori paket Homebrew
- `brew outdated` : tampilkan daftar formula dan cask yang memiliki versi lebih baru dari yang terpasang
- `brew search teks` : cari formula (paket biasa) serta cask (berkas aplikasi `.app` bagi macOS)
- `brew info formula|cask` : tampilkan informasi mengenai suatu formula atau cask (versi, lokasi pemasangan, formula/cask tambahan yang dibutuhkan, dll.)
- `brew doctor` : cek kondisi pemasangan Homebrew saat ini untuk mendeteksi kemungkinan galat atau masalah

## Links
- More information: https://docs.brew.sh/Manpage
