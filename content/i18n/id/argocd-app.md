# argocd app

Program baris perintah untuk mengatur aplikasi bersama Argo CD.

## Usage
- `argocd app list --output json|yaml|wide` : dapatkan daftar aplikasi yang diatur bersama Argo CD
- `argocd app get nama_aplikasi --output json|yaml|wide` : lihat informasi mengenai suatu aplikasi
- `argocd app create nama_aplikasi --repo alamat_url_repositori_dalam_git --path jalan/menuju/repo --dest-server https://kubernetes.default.svc --dest-namespace ns` : sebarkan (deploy) aplikasi secara internal (ke dalam klaster yang sama dengan yang dijalankan Argo CD)
- `argocd app delete nama_aplikasi` : hapus suatu aplikasi
- `argocd app set nama_aplikasi --sync-policy auto --auto-prune --self-heal` : aktifkan fitur sinkronisasi otomatis dalam suatu aplikasi
- `argocd app sync nama_aplikasi --dry-run --prune` : pratinjau hasil proses sinkronisasi aplikasi tanpa berdampak kepada klaster yang berjalan (dry-run)
- `argocd app history nama_aplikasi --output wide|id` : tampilkan riwayat penyebaran (deployment) aplikasi
- `argocd app rollback nama_aplikasi id_riwayat --prune` : batalkan penyebaran dengan memuat (rollback) versi hasil sebaran sebelumnya (dan menghapus sumber daya baru yang tak diduga), berdasarkan nomor induk (ID) riwayat

## Links
- More information: https://argo-cd.readthedocs.io/en/stable/user-guide/commands/argocd_app/
