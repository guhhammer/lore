# mongo

Program syel baris perintah lawas untuk manajemen pangkalan data (database) MongoDB. Lihat `mongosh` untuk program terbaru. Catatan: Seluruh opsi koneksi dapat digantikan dengan suatu string: `mongodb://pengguna@host:port/nama_db?authSource=nama_authdb`.

## Usage
- `mongo` : hubungkan terhadap suatu database lokal menggunakan port bawaan (`mongodb://localhost:27017`)
- `mongo --host host --port port nama_db` : hubungkan kepada suatu database
- `mongo --host host --port port --username nama_pengguna --authenticationDatabase nama_authdb nama_db` : masuk dengan akses menggunakan suatu username (nama pengguna) pada suatu database (Anda akan diminta untuk memasukkan kata sandi)
- `mongo --eval 'JSON.stringify(db.foo.findOne())' nama_db` : jalankan suatu perintah JavaScript pada suatu database

## Links
- More information: https://www.mongodb.com/docs/mongodb-shell/
