# aws cur

Buat, cari, dan hapus definisi atau spesifikasi laporan penggunaan jasa AWS.

## Usage
- `aws cur put-report-definition --report-definition file://jalan/menuju/definisi_laporan.json` : buat suatu definisi laporan biaya dan penggunaan AWS dari berkas JSON
- `aws cur describe-report-definitions` : tampilkan daftar definisi laporan penggunaan yang didefinisikan untuk akun yang sedang masuk
- `aws cur --region wilayah_aws delete-report-definition --report-name laporan` : hapus suatu definisi laporan penggunaan

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/cur/
