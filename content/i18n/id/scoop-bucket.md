# scoop bucket

Kelola bucket: Repository Git yang berisi berkas yang menjelaskan bagaimana scoop menginstall aplikasi. Jika Scoop tidak tahu di mana sebuah bucket terletak, maka lokasi repositori harus ditentukan.

## Usage
- `scoop bucket list` : tampilkan daftar semua bucket yang sedang digunakan
- `scoop bucket known` : tampilkan daftar semua bucket yang dikenal
- `scoop bucket add nama` : tambahkan bucket yang dikenal berdasarkan namanya
- `scoop bucket add nama https://example.com/repository.git` : tambahkan bucket yang tidak dikenal bersarkan nama dan URL repository Git
- `scoop bucket rm nama` : hapus suatu bucket berdasarkan namanya

## Links
- More information: https://github.com/ScoopInstaller/Scoop/wiki/Buckets
