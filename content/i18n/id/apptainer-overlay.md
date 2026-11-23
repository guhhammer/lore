# apptainer overlay

Kelola image overlay EXT3 yang dapat ditulisi untuk kontainer Apptainer.

## Usage
- `apptainer overlay create -s size jalan/menuju/image.sif` : tambahkan overlay yang dapat ditulisi ke image SIF yang sudah ada
- `apptainer overlay create -s size jalan/menuju/overlay.img` : buat image overlay EXT3 mandiri yang dapat ditulisi
- `apptainer overlay create -s size -S jalan/menuju/overlay.img` : buat image overlay sparse
- `apptainer overlay create -f -s size jalan/menuju/overlay.img` : buat overlay untuk digunakan dengan fakeroot
- `apptainer overlay create --create-dir jalan/menuju/direktori jalan/menuju/overlay.img` : buat overlay dengan direktori tertentu di dalam layout
- `apptainer overlay -h` : tampilkan bantuan

## Links
- More information: https://apptainer.org/docs/user/main/cli/apptainer_overlay.html
