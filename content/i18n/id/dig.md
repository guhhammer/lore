# dig

Alat pencari entri DNS.

## Usage
- `dig +short example.com` : cari kumpulan alamat IP yang dirujuk oleh suatu nama host (berdasarkan informasi rekor A)
- `dig +noall +answer example.com` : dapatkan jawaban lebih rinci mengenai suatu domain (berdasarkan informasi rekor A)
- `dig +short example.com A|MX|TXT|CNAME|NS` : cari informasi suatu domain menggunakan data jenis rekor DNS tertentu
- `dig +tls @1.1.1.1|8.8.8.8|9.9.9.9|... example.com` : tentukan alamat peladen pencari entri DNS alternatif dan gunakan DNS lewat TLS (DoT) secara opsional
- `dig -x 8.8.8.8` : lakukan pencarian alamat DNS terbalik terhadap suatu alamat IP (berdasarkan informasi rekor PTR)
- `dig +nssearch example.com` : cari daftar peladen nama otoritatif untuk zona DNS tertentu dan tampilkan informasi rekor SOA
- `dig +trace example.com` : lakukan pencarian secara iteratif dan tampilkan jejak penelusuran untuk menuntaskan pencarian suatu nama domain
- `dig +tcp -p port @alamat_ip_peladen_dns example.com` : lakukan pencarian menggunakan suatu peladen DNS dengan port non-standar dan protokol TCP

## Links
- More information: https://manned.org/dig
