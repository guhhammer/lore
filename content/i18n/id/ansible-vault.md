# ansible-vault

Lakukan proses enkripsi (persandian) dan dekripsi terhadap nilai, struktur data, dan berkas dalam proyek Ansible.

## Usage
- `ansible-vault create jalan/menuju/berkas_brankas` : buat suatu berkas brankas terenkripsi baru dengan memasukkan kata sandi pada prompt masukan berikutnya
- `ansible-vault edit|view|rekey jalan/menuju/berkas_brankas` : sunting, lihat, atau ganti kunci enkripsi terhadap suatu berkas brankas terenkripsi dengan memasukkan kata sandi pada prompt masukan berikutnya
- `ansible-vault create --vault-password-file jalan/menuju/berkas_kata_sandi jalan/menuju/berkas_brankas` : buat suatu berkas brankas terenkripsi baru menggunakan berkas kunci (kata sandi) brankas untuk mengenkripsinya
- `ansible-vault encrypt --vault-password-file jalan/menuju/berkas_kata_sandi jalan/menuju/berkas` : sandikan suatu berkas teks yang ada menggunakan berkas kata sandi opsional
- `ansible-vault encrypt_string` : sandikan suatu teks string menggunakan format string terenkripsi standar Ansible, dan menampilkan petunjuk secara interaktif
- `ansible-vault view --vault-password-file jalan/menuju/berkas_kata_sandi jalan/menuju/berkas_brankas` : lihat isi suatu brankas yang terenkripsi, menggunakan berkas kata sandi untuk membukanya
- `ansible-vault rekey --vault-password-file jalan/menuju/berkas_kata_sandi_lama --new-vault-password-file jalan/menuju/berkas_kata_sandi_baru jalan/menuju/berkas_brankas` : ganti kunci (kata sandi) pada brankas terenkripsi dengan membuat suatu berkas kata sandi baru

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/vault_guide/index.html
