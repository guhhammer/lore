# docker container stats

Siarkan statistik penggunaan sumber daya kontainer.

## Usage
- `docker stats` : siarkan statistik untuk seluruh kontainer berjalan
- `docker stats kontainer1 kontainer2 ...` : siatkan statistik untuk satu atau beberapa kontainer tertentu
- `docker stats --format "\{\{.Name\}\}:\t\{\{.CPUPerc\}\}"` : ubah format kolom luaran untuk menampilkan persentase penggunaan CPU oleh kontainer
- `docker stats -a` : siarkan statistik untuk semua kontainer (baik yang dijalankan atau dihentikan)
- `docker stats --no-stream` : jangan siarkan namun tampilkan statistik penggunaan saat ini

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/stats/
