# bq

Alat pemrosesan data berbasis Python untuk BigQuery, layanan pergudangan data Google Cloud yang sepenuhnya terkelola dan bersifat serverless.

## Usage
- `bq query --nouse_legacy_sql --dry_run 'SELECT COUNT(*) FROM NAMA_DATASET.NAMA_TABEL'` : jalankan suatu perintah kueri terhadap suatu tabel BigQuery dalam format SQL dasar, tambahkan opsi `--dry_run` untuk menaksir jumlah bita yang akan dibaca pada proses eksekusi
- `bq query --use_legacy_sql=false --parameter='ts_value:TIMESTAMP:2016-12-07 08:00:00' 'SELECT TIMESTAMP_ADD(@ts_value, INTERVAL 1 HOUR)'` : jalankan suatu perintah kueri dengan kumpulan parameter
- `bq mk --location=US nama_dataset.nama_tabel` : buat suatu dataset atau tabel pada wilayah layanan Amerika Serikat (US)
- `bq ls --filter labels.key:nilai --max_results integer --format=prettyjson --project_id id_proyek` : tampilkan seluruh dataset pada suatu proyek
- `bq load --location lokasi --source_format CSV|JSON|PARQUET|AVRO dataset.table jalan_menuju_sumber` : lakukan proses pemuatan data secara batch dari berkas tertentu dalam format seperti CSV, JSON, Parquet, dan Avro ke dalam suatu tabel
- `bq cp dataset.TABEL_LAMA dataset.tabel_baru` : salin suatu tabel menuju tabel lainnya
- `bq help` : tampilkan bantuan

## Links
- More information: https://docs.cloud.google.com/bigquery/docs/reference/bq-cli-reference
