# acme.sh-dns

Gunakan verifikasi DNS-01 untuk menerbitkan sertifikat HTTPS.

## Usage
- `acme.sh --issue --dns api_dns --domain example.com` : terbitkan sertifikat menggunakan metode verifikasi DNS via API (daftar jenis `api_dns` yang didukung tersedia dalam <https://github.com/acmesh-official/acme.sh/wiki/How-to-use-lexicon-DNS-API>)
- `acme.sh --issue --dns api_dns --domain example.com --domain *.example.com` : terbitkan sertifikat wildcard (ditandai dengan tanda bintang) menggunakan verifikasi DNS via API
- `acme.sh --issue --dns api_dns --domain example.com --challenge-alias alias-untuk-verifikasi-example.com` : terbitkan sertifikat menggunakan verifikasi DNS alias
- `acme.sh --issue --dns api_dns --domain example.com --dnssleep 300` : terbitkan sertifikat dengan masa tunggu pemutakhiran DNS yang ditentukan (dalam detik), sehingga `acme.sh` tidak melakukan proses validasi DNS otomatis melalui server DNS Cloudflare/Google
- `acme.sh --issue --dns --domain example.com --yes-I-know-dns-manual-mode-enough-go-ahead-please` : terbitkan sertifikat menggunakan verifikasi DNS manual

## Links
- More information: https://github.com/acmesh-official/acme.sh/wiki
