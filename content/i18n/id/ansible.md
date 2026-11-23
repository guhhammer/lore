# ansible

Atur grup perangkat komputer yang secara jarak jauh melalui SSH. (Gunakan berkas `/etc/ansible/hosts` untuk menambahkan grup atau host baru). Beberapa subperintah seperti `ansible galaxy` mempunyai dokumentasi terpisah.

## Usage
- `ansible grup --list-hosts` : tampilkan daftar host yang tergabung dalam suatu grup
- `ansible grup -m ping` : uji koneksi (ping) kepada grup perangkat tertentu dengan menggunakan modul ping
- `ansible grup -m setup` : tampilkan informasi faktual tentang suatu grup perangkat dengan menggunakan modul setup
- `ansible grup -m command -a 'perintah_saya'` : jalankan perintah pada suatu kelompok perangkat melalui modul command dengan kumpulan argumen
- `ansible grup -b --ask-become-pass -m command -a 'perintah_saya'` : jalankan perintah dengan hak akses administratif
- `ansible grup -i file_inventaris -m command -a 'perintah_saya'` : jalankan perintah menggunakan berkas inventaris tertentu
- `ansible localhost -m debug -a 'var=groups.keys()'` : tampilkan daftar grup dalam sebuah inventaris

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible.html
