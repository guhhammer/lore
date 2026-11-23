# composer

Manajer paket untuk proyek PHP.

## Usage
- `composer init` : buat sebuah berkas `composer.json` secara interaktif
- `composer require nama_pengguna/nama_paket` : tambahkan paket sebagai sebuah pustaka prasyarat (dependency) untuk proyek ini, menambahkan ke `composer.json`
- `composer install` : pasang seluruh prasyarat dalam `composer.json` proyek ini dan buat berkas `composer.lock`
- `composer remove nama_pengguna/nama_paket` : hapus pemasangan suatu paket dalam proyek ini, sehingga menghapusnya dari entri prasyarat pada `composer.json` dan `composer.lock`
- `composer update` : perbarui semua pustaka prasyarat dalam `composer.json` proyek ini dan catat versi-versi terkini dalam berkas `composer.lock`
- `composer update --lock` : memperbarui `composer.lock` setelah mengubah `composer.json` secara manual
- `composer why-not user/nama_paket` : cari tahu alasa mengapa sebuah dependensi tidak dapat dipasang
- `composer self-update` : perbarui program composer menuju versi terbaru

## Links
- More information: https://getcomposer.org/doc/03-cli.md
