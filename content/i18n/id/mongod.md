# mongod

Peladen basis data MongoDB.

## Usage
- `mongod --dbpath jalan/menuju/direktori` : tentukan direktori penyimpanan basis data (default: `/data/db` dalam Linux dan macOS, `C:\data\db` dalam Windows)
- `mongod --config jalan/menuju/berkas` : tentukan berkas konfigurasi basis data
- `mongod --port port` : tentukan port yang akan digunakan (default: 27017)
- `mongod --profile 0|1|2` : tentukan tingkat pencatatan perilaku (profiling) peladen basis data. 0 mati, 1 hanya operasi lambat, 2 semuanya (default: 0)

## Links
- More information: https://www.mongodb.com/docs/manual/reference/program/mongod/
