# aws cognito-idp

Atur suatu kumpulan pengguna (user pool) Amazon Cognito beserta pengguna dan grup di dalamnya, dan lakukan autentikasi.

## Usage
- `aws cognito-idp create-user-pool --pool-name nama` : buat suatu kumpulan pengguna (pool) Cognito baru
- `aws cognito-idp list-user-pools --max-results 10` : tampilkan seluruh kumpulan pengguna
- `aws cognito-idp delete-user-pool --user-pool-id id_kumpulan_pengguna` : hapus suatu kumpulan pengguna
- `aws cognito-idp admin-create-user --username nama_pengguna --user-pool-id id_kumpulan_pengguna` : buat suatu pengguna baru dalam suatu kumpulan
- `aws cognito-idp list-users --user-pool-id id_kumpulan_pengguna` : tampilkan daftar pengguna dalam suatu kumpulan
- `aws cognito-idp admin-delete-user --username nama_pengguna --user-pool-id id_kumpulan_pengguna` : hapus suatu pengguna dari suatu kumpulan

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/cognito-idp/
