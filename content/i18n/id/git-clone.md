# git clone

Gandakan repositori dari lokasi luar/remote menuju lokal.

## Usage
- `git clone lokasi_repositori_remote jalan/menuju/direktori` : gandakan repositori yang ada ke direktori tertentu
- `git clone --recursive lokasi_repositori_remote` : gandakan repositori yang ada dan submodulnya
- `git clone -n lokasi_repositori_remote` : gandakan hanya direktori `.git` pada repositori saat ini
- `git clone -l jalan/menuju/repositori/lokal` : gandakan repositori lokal
- `git clone -q lokasi_repositori_remote` : gandakan dengan senyap
- `git clone --depth 10 lokasi_repositori_remote` : gandakan repositori yang sudah ada dengan hanya mengambil 10 komit paling baru pada branch bawaan (berguna untuk menghemat waktu)
- `git clone -b name --single-branch lokasi_repositori_remote` : gandakan repositori yang sudah ada dengan hanya mengambil dari cabang tertentu
- `git clone -c core.sshCommand="ssh -i jalan/menuju/kunci_ssh_privat" lokasi_repositori_remote` : gandakan repositori yang sudah ada menggunakan perintah SSH tertentu

## Links
- More information: https://git-scm.com/docs/git-clone
