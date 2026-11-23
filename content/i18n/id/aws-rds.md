# aws rds

Gunakan AWS Relational Database Service, suatu layanan untuk membuat, menjalankan, dan membesarkan operasional sistem pangkalan data relasional.

## Usage
- `aws rds subperintah help` : tampilkan bantuan untuk suatu subperintah RDS
- `aws rds stop-db-instance --db-instance-identifier nomor_induk_instansi` : hentikan suatu instansi RDS
- `aws rds start-db-instance --db-instance-identifier nomor_induk_instansi` : jalankan suatu instansi RDS
- `aws rds modify-db-instance --db-instance-identifier nomor_induk_instansi kumpulan_parameter --apply-immediately` : ubah suatu instansi RDS
- `aws rds apply-pending-maintenance-action --resource-identifier arn_pangkalan_data --apply-action system-update --opt-in-type immediate` : terapkan pembaruan untuk suatu instansi RDS
- `aws rds modify-db-instance --db-instance-identifier nomor_induk_lama_instansi --new-db-instance-identifier nomor_induk_baru_instansi` : ubah nomor induk suatu instansi
- `aws rds reboot-db-instance --db-instance-identifier nomor_induk_instansi` : nyalakan ulang suatu instansi
- `aws rds delete-db-instance --db-instance-identifier nomor_induk_instansi --final-db-snapshot-identifier nomor_induk_snapshot --delete-automated-backups` : hapus suatu instansi

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/rds/
