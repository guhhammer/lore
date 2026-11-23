# yarn

Pengelola paket alternatif untuk JavaScript dan Node.js.

## Usage
- `yarn global add nama_modul` : pasang suatu modul secara global
- `yarn install` : pasang semua pustaka prasyarat (dependency) yang dirujuk dalam berkas `package.json` (perintah `install` adalah opsional)
- `yarn add nama_modul@versi` : pasang dan catat suatu modul sebagai prasyarat ke dalam berkas `package.json` (tambahkan `--dev` jika hendak menyimpannya sebagai prasyarat khusus tahap pengembangan)
- `yarn remove nama_modul` : hapus pemasangan modul beserta entrinya dalam berkas `package.json`
- `yarn init` : membuat berkas `package.json` secara interaktif
- `yarn why nama_modul` : periksa apakah suatu modul merupakan suatu prasyarat serta tampilkan daftar modul lainnya yang bergantung kepadanya

## Links
- More information: https://yarnpkg.com/cli
