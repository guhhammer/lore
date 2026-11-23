# apptainer build

Buat image container Apptainer.

## Usage
- `apptainer build jalan/menuju/image.sif jalan/menuju/definisi.def` : buat container dari sebuah file definisi
- `apptainer build jalan/menuju/image.sif docker://image:tag` : buat container dari Docker Hub
- `apptainer build jalan/menuju/image.sif library://user/collection/container:tag` : buat container dari Container Library
- `apptainer build -s jalan/menuju/direktori docker://image:tag` : buat direktori sandbox yang dapat ditulis alih-alih file image
- `apptainer build --disable-cache jalan/menuju/image.sif docker://image:tag` : buat container tanpa menggunakan cache
- `apptainer build -F jalan/menuju/image.sif jalan/menuju/definition.def` : paksa timpa file image yang sudah ada
- `apptainer build -f jalan/menuju/image.sif jalan/menuju/definition.def` : buat menggunakan fakeroot untuk build unprivileged
- `apptainer build -h` : tampilkan bantuan

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_build.html
