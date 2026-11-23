# aptitude

Alat manajer paket piranti lunak Ubuntu dan Debian.

## Usage
- `sudo aptitude update` : padankan seluruh informasi daftar paket dan versi yang tersedia. Perintah ini seharusnya dijalankan terlebih dahulu sebelum memasukkan perintah `aptitude` lainnya
- `sudo aptitude install paket` : pasang baru suatu paket beserta ketergantungannya
- `aptitude search paket` : cari untuk suatu paket
- `aptitude search '?installed(paket)'` : cari untuk paket yang telah terpasang: (`?installed` adalah kata kunci pencarian dalam `aptitude`)
- `sudo aptitude remove paket` : hapus suatu paket beserta seluruh ketergantungannya
- `sudo aptitude upgrade` : mutakhirkan seluruh paket yang telah terpasang menuju versi terbaru bila tersedia
- `sudo aptitude full-upgrade` : mutakhirkan seluruh paket (layaknya `aptitude upgrade`) serta buang paket-paket yang sudah tak terpakai dan pasang paket tambahan untuk memenuhi syarat ketergantungan paket yang terbaru
- `sudo aptitude hold '?installed(paket)'` : masukkan suatu paket terpasang ke dalam daftar penahanan sehingga tidak boleh diperbarui secara otomatis

## Links
- More information: https://manned.org/aptitude
