# aws lambda

Gunakan AWS Lambda, suatu layanan komputasi untuk menjalankan kode perintah tanpa membuat atau mengatur infrastruktur peladen.

## Usage
- `aws lambda invoke --function-name nama jalan/menuju/respons.json` : jalankan suatu function
- `aws lambda invoke --function-name nama --payload json jalan/menuju/respons.json` : jalankan suatu function dengan payload masukan (input) dalam format JSON
- `aws lambda list-functions` : tampilkan daftar function yang tersedia
- `aws lambda get-function-configuration --function-name nama` : tampilkan informasi konfigurasi mengenai suatu function
- `aws lambda list-aliases --function-name nama` : tampilkan daftar alias yang didaftarkan terhadap suatu function
- `aws lambda get-function-concurrency --function-name nama` : tampilkan informasi konfigurasi konkurensi yang dicadangkan (reserved concurrency) terhadap suatu function
- `aws lambda get-policy --function-name nama` : tampilkan daftar layanan AWS yang berhak memanggil suatu function

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/lambda/
