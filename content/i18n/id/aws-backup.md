# aws backup

Layanan pencadangan untuk melindungi kumpulan layanan dan data terkait dalam Amazon Web Services.

## Usage
- `aws backup get-backup-plan --backup-plan-id nomor_induk` : tampilkan rincian BackupPlan (rencana pemulihan layanan yang dicadangkan) berdasarkan nomor induknya
- `aws backup create-backup-plan --backup-plan rencana_pencadangan` : buat suatu BackupPlan dengan nama dan aturan tertentu
- `aws backup delete-backup-plan --backup-plan-id nomor_induk` : hapus suatu BackupPlan berdasarkan nomor induknya
- `aws backup list-backup-plans` : tampilkan seluruh BackupPlan yang aktif dalam akun saat ini
- `aws backup list-report-jobs` : tampilkan rincian atas laporan pekerjaan pencadangan Anda

## Links
- More information: https://docs.aws.amazon.com/cli/latest/reference/backup/
