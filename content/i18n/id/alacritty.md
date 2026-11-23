# alacritty

Lintas platform, terakselerasi GPU terminal emulator.

## Usage
- `alacritty` : buka jendela Alacritty baru
- `alacritty --daemon` : jalankan Alacritty sebagai peladen daemon (tanpa membuat jendela baru)
- `alacritty msg create-window` : buat jendela baru pada proses Alacritty yang sedang berjalan
- `alacritty --working-directory jalan/menuju/direktori` : jalankan Alacritty pada direktori tertentu (dapat juga bekerja dalam `alacritty msg create-window`)
- `alacritty -e perintah` : jalankan perintah di jendela Alacritty baru (dapat juga bekerja dalam `alacritty msg create-window`)
- `alacritty --config-file jalan/menuju/konfigurasi.toml` : gunakan berkas konfigurasi alternatif untuk memuat program (nilai default `$XDG_CONFIG_HOME/alacritty/alacritty.toml`)

## Links
- More information: https://manned.org/alacritty
