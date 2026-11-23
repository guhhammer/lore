# aws

Alat baris perintah (CLI) resmi untuk Amazon Web Services. Beberapa subperintah seperti `s3` mempunyai dokumentasi terpisah.

## Usage
- `aws configure wizard` : atur penggunaan AWS Command-line secara umum
- `aws configure sso` : atur penggunaan AWS Command-line menggunakan SSO
- `aws sts get-caller-identity` : dapatkan identitas pemanggil perintah (digunakan untuk menyelesaikan permasalahan yang berkaitan dengan hak akses)
- `aws dynamodb list-tables --region us-east-1 --output yaml` : tampilkan daftar sumber daya AWS dalam suatu wilayah (region) ke dalam format YAML
- `aws iam create-user --cli-auto-prompt` : gunakan fitur auto prompt untuk membantu melakukan suatu perintah
- `aws dynamodb wizard new_table` : jalankan perintah wizard terhadap suatu sumber daya AWS
- `aws dynamodb update-table --generate-cli-skeleton` : buat suatu berkas JSON CLI Skeleton (berguna untuk mengatur infrastruktur sebagai perintah kode)
- `aws perintah help` : tampilkan bantuan terhadap suatu perintah

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/
