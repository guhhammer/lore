# aws iam

Berinteraksi dengan Identity and Access Management (IAM), manajer untuk mengelola identitas dan akses layanan AWS secara aman.

## Usage
- `aws iam list-users` : tampilkan daftar pengguna
- `aws iam list-policies` : tampilkan daftar kebijakan
- `aws iam list-groups` : tampilkan daftar grup
- `aws iam get-group --group-name nama_grup` : dapatkan daftar pengguna dalam suatu grup
- `aws iam get-policy --policy-arn arn:aws:iam::aws:policy/nama_kebijakan` : tampilkan deskripsi suatu kebijakan IAM
- `aws iam list-access-keys` : tampilkan daftar kunci akses
- `aws iam list-access-keys --user-name nama_pengguna` : tampilkan daftar kunci akses untuk suatu pengguna
- `aws iam help` : tampilkan bantuan

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/iam/
