# apptainer

Kelola container untuk HPC dan komputasi ilmiah. Beberapa subperintah seperti `build`, `pull`, dan `push` memiliki dokumentasi penggunaan tersendiri.

## Usage
- `apptainer pull jalan/menuju/image.sif docker://image:tag` : unduh container dari Docker Hub
- `apptainer pull jalan/menuju/image.sif library://pengguna/koleksi/container:tag` : unduh container dari Container Library
- `apptainer build jalan/menuju/image.sif jalan/menuju/definisi.def` : buat container dari file definisi
- `apptainer shell jalan/menuju/image.sif` : mulai shell interaktif di dalam container
- `apptainer exec jalan/menuju/image.sif perintah` : jalankan perintah di dalam container
- `apptainer run jalan/menuju/image.sif` : jalankan runscript setelan standar (default) dari sebuah container
- `apptainer inspect jalan/menuju/image.sif` : periksa metadata dari sebuah container
- `apptainer -h` : tampilkan bantuan

## Links
- More information: https://apptainer.org/docs/user/main/cli.html
