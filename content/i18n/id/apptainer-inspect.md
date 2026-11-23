# apptainer inspect

Tampilkan metadata dari image kontainer Apptainer.

## Usage
- `apptainer inspect jalan/menuju/image.sif` : tampilkan label dari sebuah image (bawaan)
- `apptainer inspect -d jalan/menuju/image.sif` : tampilkan file definisi yang digunakan untuk membangun image
- `apptainer inspect -r jalan/menuju/image.sif` : tampilkan runscript untuk suatu image
- `apptainer inspect -e jalan/menuju/image.sif` : tampilkan variabel environment dari suatu image
- `apptainer inspect --list-apps jalan/menuju/image.sif` : tampilkan daftar semua aplikasi di dalam kontainer
- `apptainer inspect --all jalan/menuju/image.sif` : tampilkan semua data yang tersedia dalam format JSON
- `apptainer inspect -h` : tampilkan bantuan

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_inspect.html
