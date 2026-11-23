# amass enum

Cari seluruh subdomain dari suatu domain internet.

## Usage
- `amass enum -d nama_domain` : cari para subdomain dari suatu domain (secara pasif)
- `amass enum -active -d nama_domain -p 80,443,8080` : cari para subdomain dari domain dengan memeriksa apakah subdomain tersebut dapat ditemukan
- `amass enum -brute -d nama_domain` : lakukan pencarian terhadap para subdomain secara paksa (brute force)
- `amass enum -o berkas_output -d nama_domain` : simpan hasil pencarian ke dalam suatu berkas teks
- `amass enum -o berkas_output -dir jalan/menuju/direktori -d nama_domain` : simpan hasil luaran (output) terminal menuju suatu berkas dan informasi tambahan ke dalam direktori tertentu
- `amass enum -list` : tampilkan daftar sumber pencarian data
