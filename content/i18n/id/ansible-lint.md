# ansible-lint

Terapkan aturan dan rekomendasi praktik terbaik terhadap konten otomasi Anda.

## Usage
- `ansible-lint jalan/menuju/berkas_playbook jalan/menuju/direktori_peran` : terapkan terhadap suatu playbook dan direktori peran
- `ansible-lint -x peraturan1,peraturan2,... jalan/menuju/berkas_playbook` : terapkan terhadap suatu playbook kecuali kumpulan peraturan tertentu
- `ansible-lint -o -p jalan/menuju/berkas_playbook` : terapkan terhadap suatu playbook dalam mode luring (offline), dan atur luaran perintah dalam format PEP8
- `ansible-lint -r jalan/menuju/direktori_peraturan_kustom jalan/menuju/berkas_playbook` : terapkan terhadap suatu playbook menggunakan suatu direktori peraturan
- `ansible-lint jalan/menuju/direktori_proyek` : terapkan terhadap seluruh konten Ansible, secara rekursif, dalam suatu direktori

## Links
- More information: https://docs.ansible.com/projects/lint/
