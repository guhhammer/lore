# brew bundle

Pembungkus untuk Homebrew, Homebrew Cask, dan App Store untuk macOS.

## Usage
- `brew bundle` : pasang seluruh paket menurut data Brewfile pada direktori saat ini
- `brew bundle --file jalan/menuju/berkas` : pasang seluruh paket menurut data Brewfile pada lokasi tertentu
- `brew bundle dump` : buat suatu berkas Brewfile berisikan daftar seluruh paket yang terpasang saat ini
- `brew bundle cleanup --force` : hapus seluruh formula yang tidak didefinisikan atau dibutuhkan pada formula dalam berkas Brewfile
- `brew bundle check` : cari tahu apakah terdapat formula yang perlu dipasang atau dimutakhirkan dalam berkas Brewfile
- `brew bundle list --all` : tampilkan seluruh entri dalam berkas Brewfile

## Links
- More information: https://docs.brew.sh/Manpage#bundle-subcommand
