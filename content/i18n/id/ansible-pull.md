# ansible-pull

Tarik buku-buku aturan main (playbook) dari suatu repositori sistem manajemen versi (VCS), dan jalankan tugas-tugasnya bagi host lokal.

## Usage
- `ansible-pull -U url_repositori` : tarik suatu playbook dari repositori VCS, kemudian jalankan aturan default dari playbook `local.yml`
- `ansible-pull -U url_repositori playbook` : tarik suatu playbook dari repositori VCS, kemudian jalankan aturan playbook dengan nama tertentu
- `ansible-pull -U url_repositori -C cabang playbook` : tarik suatu playbook dari Cabang tertentu pada repositori VCS, kemudian jalankan aturan playbook dengan nama tertentu
- `ansible-pull -U url_repositori -i berkas_hosts playbook` : tarik suatu playbook dari repositori VCS, kemudian definisikan daftar perangkat/host dari suatu berkas (hosts), kemudian jalankan aturan playbook dengan nama tertentu

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-pull.html
