# set-location

Tampilkan direktori kerja saat ini atau pindah ke direktori lain. Catatan: Perintah ini hanya dapat digunakan dalam PowerShell.

## Usage
- `Set-Location jalan\menuju\direktori` : pergi menuju suatu direktori pada drive yang sama
- `Set-Location C:jalan\menuju\direktori` : pergi menuju direktori tertentu pada drive yang berbeda
- `Set-Location jalan\menuju\direktori -PassThru` : pergi dan tampilkan lokasi lengkap (absolute path) atas direktori yang dituju
- `Set-Location ..` : pergi menuju induk dari direktori dari saat ini
- `Set-Location ~` : pergi menuju direktori pangkal/home milik pengguna saat ini
- `Set-Location -|+` : pergi menuju direktori yang telah dikunjungi sebelumnya/setelahnya
- `Set-Location \` : pergi menuju akar (root) dari drive saat ini
