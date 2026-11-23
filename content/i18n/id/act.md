# act

Jalankan GitHub Actions secara lokal melalui Docker.

## Usage
- `act -l` : tampilkan daftar actions (tugas dalam GitHub Actions) yang tersedia
- `act` : jalankan tugas dengan event default
- `act jenis_event` : jalankan event pemicu tugas tertentu
- `act -j job_id` : jalankan tugas/job tertentu
- `act -n` : tampilkan tugas-tugas yang akan dijalankan tanpa mengeksekusikannya (dry-run)
- `act -v` : tampilkan log tingkat verbose
- `act push -W jalan/menuju/workflow` : jalankan Workflow tertentu

## Links
- More information: https://manned.org/act
