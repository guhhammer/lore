# bundle

Manajer paket dan ketergantungan untuk bahasa pemrograman Ruby.

## Usage
- `bundle install` : pasang seluruh gem (paket) yang terdaftar dalam berkas `Gemfile` yang diharapkan tersedia dalam direktori kerja saat ini
- `bundle exec perintah kumpulan_argumen_perintah` : jalankan suatu perintah dalam konteks lingkungan pemasangan paket Ruby saat ini
- `bundle update --all` : perbarui seluruh gem menurut aturan ketergantungan dalam berkas `Gemfile` dan bangun ulang berkas `Gemfile.lock`
- `bundle update nama_gem1 nama_gem2 ...` : perbarui satu atau beberapa gem yang terdaftar dalam berkas `Gemfile`
- `bundle update --patch nama_gem1 nama_gem2 ...` : hanya perbarui kumpulan gem `Gemfile` menuju versi patch/minor berikutnya
- `bundle update --group development` : perbarui seluruh gem dalam grup ketergantungan tertentu dalam `Gemfile`
- `bundle outdated` : tampilkan kumpulan gem yang terpasang dalam `Gemfile` beserta versi terbaru yang tersedia untuk pemutakhiran
- `bundle gem nama_gem` : buat suatu kerangka paket/gem baru

## Links
- More information: https://bundler.io/man/bundle.1.html
