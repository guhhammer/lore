# aws glue

Program baris perintah untuk AWS Glue. Tentukan endpoint publik untuk layanan AWS Glue.

## Usage
- `aws glue list-jobs` : tampilkan daftar pekerjaan
- `aws glue start-job-run --job-name nama_pekerjaan` : jalankan suatu pekerjaan
- `aws glue start-workflow-run --name nama_alur_kerja` : jalankan suatu alur kerja (workflow)
- `aws glue list-triggers` : tampilkan daftar pemicu pekerjaan
- `aws glue start-trigger --name nama_pemicu` : jalankan suatu pemicu pekerjaan
- `aws glue create-dev-endpoint --endpoint-name nama --role-arn role_arn_yang_digunakan_dalam_endpoint` : buat suatu endpoint versi pengembangan

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/glue/
