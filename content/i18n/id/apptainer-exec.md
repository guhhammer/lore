# apptainer exec

Jalankan perintah di dalam container Apptainer.

## Usage
- `apptainer exec path/ke/image.sif perintah` : jalankan perintah di dalam container
- `apptainer exec path/ke/image.sif perintah arg1 arg2 ...` : jalankan perintah beserta argumen
- `apptainer exec -B jalan/menuju/sumber:jalan/menuju/tujuan jalan/menuju/image.sif perintah` : jalankan perintah dengan bind mount dari host ke container
- `apptainer exec --env variable=nilai jalan/menuju/image.sif perintah` : jalankan perintah dengan variabel environment
- `apptainer exec -C jalan/menuju/image.sif perintah` : jalankan perintah dalam mode terisolasi penuh (filesystem terisolasi, PID, IPC, dan environment yang bersih)
- `apptainer exec --writable-tmpfs jalan/menuju/image.sif perintah` : jalankan perintah dengan overlay filesystem sementara yang bisa ditulis
- `apptainer exec --nv jalan/menuju/image.sif perintah` : jalankan perintah dengan dukungan GPU NVIDIA
- `apptainer exec -h` : tampilkan bantuan

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_exec.html
