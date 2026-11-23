# adig

Cetak informasi yang diterima dari server Domain Name System (DNS).

## Usage
- `adig example.com` : tampilkan record A (bawaan) dari DNS untuk hostname
- `adig -d example.com` : tampilkan output debugging tambahan
- `adig -s 1.2.3.4 example.com` : hubungkan ke server DNS tertentu
- `adig -T port example.com` : gunakan port TCP tertentu untuk terhubung ke server DNS
- `adig -U port example.com` : gunakan port UDP tertentu untuk terhubung ke server DNS

## Links
- More information: https://manned.org/adig
