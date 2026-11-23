# apptainer delete

Hapus image kontainer dari library remote.

## Usage
- `apptainer delete library://user/collection/container:tag` : hapus image dari Container Library
- `apptainer delete -A amd64|arm64|ppc64le library://user/collection/container:tag` : hapus image untuk arsitektur tertentu
- `apptainer delete -F library://user/collection/container:tag` : hapus paksa image tanpa konfirmasi
- `apptainer delete --library https://library.example.com library://user/collection/container:tag` : hapus image dari server library tertentu
- `apptainer delete --no-https library://hostname/user/collection/container:tag` : hapus image menggunakan HTTP alih-alih HTTPS
- `apptainer delete -h` : tampilkan bantuan

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_delete.html
