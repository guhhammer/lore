# aws quicksight

Buat, hapus, tampilkan daftar, cari, dan perbarui entitas-entitas AWS QuickSight.

## Usage
- `aws quicksight list-data-sets --aws-account-id id_akun_aws` : tampilkan kumpulan dataset
- `aws quicksight list-users --aws-account-id id_akun_aws --namespace default` : tampilkan daftar pengguna
- `aws quicksight list-groups --aws-account-id id_akun_aws --namespace default` : tampilkan daftar kelompok
- `aws quicksight list-dashboards --aws-account-id id_akun_aws` : tampilkan daftar dasbor
- `aws quicksight describe-data-set --aws-account-id id_akun_aws --data-set-id id_data_set` : tampilkan informasi terperinci mengenai suatu dataset
- `aws quicksight describe-data-set-permissions --aws-account-id id_akun_aws --data-set-id id_data_set` : tampilkan siapa yang mendapat hak akses dan hak kegiatan terhadap suatu dataset

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/quicksight/
