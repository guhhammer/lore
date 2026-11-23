# ansible-galaxy

Buat dan atur peran pengguna (role) Ansible.

## Usage
- `ansible-galaxy role|collection list` : tampilkan daftar peran atau koleksi yang tersedia
- `ansible-galaxy role search nama_peran -vvvvvv` : cari suatu peran berdasarkan nama menggunakan tingkat verbositas tertentu (`-v` harus dimasukkan pada akhir baris perintah)
- `ansible-galaxy role install|remove nama_peran1 nama_peran2 ...` : pasang atau bongkar peran-peran pengguna
- `ansible-galaxy role init nama_peran` : terbitkan sebuah peran baru
- `ansible-galaxy role info nama_peran` : dapatkan informasi mengenai suatu peran
- `ansible-galaxy collection install|remove nama_koleksi1 nama_koleksi1 ...` : pasang atau bongkar kumpulan koleksi (collection)
- `ansible-galaxy role|collection -h` : tampilkan bantuan mengenai manajemen peran (role) maupun koleksi (collection)

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-galaxy.html
