# aws kinesis

Berinteraksi dengan Amazon Kinesis Data Streams, suatu layanan yang memudahkan pemrosesan penyiaran big data secara segera dan elastis.

## Usage
- `aws kinesis list-streams` : tampilkan daftar seluruh siaran dalam akun yang masuk
- `aws kinesis put-record --stream-name nama --partition-key kunci --data pesan_dalam_sandian_base64` : tulis sebuah rekor dalam suatu siaran Kinesis
- `aws kinesis put-record --stream-name nama --partition-key kunci --data "$( echo "pesan mentah saya" | base64 )"` : tulis sebuah rekor dalam suatu siaran Kinesis dengan menyandikan pesan Base64 secara langsung dalam barisan perintah
- `aws kinesis list-shards --stream-name nama` : tampilkan daftar pecahan (shard) yang tersedia dalam suatu siaran
- `aws kinesis get-shard-iterator --shard-iterator-type TRIM_HORIZON --stream-name nama --shard-id id_pecahan` : dapatkan sebuah iterator pecahan untuk membaca dari pesan terlampau dalam suatu pecahan siaran
- `aws kinesis get-records --shard-iterator iterator` : baca kumpulan rekor dari suatu pecahan, menggunakan iterator pecahan
