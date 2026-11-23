# ansible-playbook

Jalankan kumpulan tugas yang didefinisikan di dalam buku aturan main (playbook), kepada para mesin secara jarak jauh melalui SSH.

## Usage
- `ansible-playbook playbook` : jalankan kumpulan tugas yang didefinisikan dalam buku aturan main (playbook)
- `ansible-playbook playbook -i berkas_inventaris` : jalankan kumpulan tugas playbook dengan inventaris mesin secara kustom
- `ansible-playbook playbook -e "variabel1=nilai1 variabel2=nilai2"` : jalankan kumpulan tugas playbook dengan variabel ekstra sebagaimana didefinisikan dalam barisan perintah (command-line)
- `ansible-playbook playbook -e "@daftar_variabel.json"` : jalankan kumpulan tugas playbook dengan variabel ekstra sebagaimana didefinisikan di dalam suatu berkas JSON
- `ansible-playbook playbook -t tag1,tag2` : jalankan kumpulan tugas playbook dengan konfigurasi tag tertentu
- `ansible-playbook playbook --start-at-task nama_tugas` : jalankan kumpulan tugas playbook, dimulai dari nama tugas spesifik
- `ansible-playbook playbook -C -D` : jalankan kumpulan tugas playbook tanpa melakukan perubahan sebenarnya (dry-run)

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-playbook.html
