# ansible-doc

Tampilkan informasi mengenai modul-modul (action plugins) yang terpasang dalam pustaka pemasangan Ansible. Tampilkan informasi singkat mengenai daftar plugin beserta deskripsi singkatnya.

## Usage
- `ansible-doc -l` : tampilkan daftar modul/plugin yang tersedia
- `ansible-doc -t become|cache|callback|cliconf|connection|... -l` : tampilkan daftar modul/plugin berdasarkan jenisnya
- `ansible-doc nama_plugin` : tampilkan informasi mengenai suatu modul/plugin
- `ansible-doc -t become|cache|callback|cliconf|connection|... nama_plugin` : tampilkan informasi mengenai suatu modul/plugin berdasarkan jenis spesifiknya
- `ansible-doc -s nama_plugin` : lihat contoh cara penggunaan (dalam playbook) bagi suatu modul/plugin
- `ansible-doc -j nama_plugin` : tampilkan informasi mengenai suatu plugin/modul dalam format JSON

## Links
- More information: https://docs.ansible.com/projects/ansible/latest/cli/ansible-doc.html
