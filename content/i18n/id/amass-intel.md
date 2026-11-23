# amass intel

Kumpulkan data pendukung pengintaian bersumber terbuka (OSI) terhadap suatu organisasi, seperti domain pangkal dan informasi Nomor Sistem Otonom (ASN).

## Usage
- `amass intel -addr 192.168.0.1-254` : cari para domain pangkal yang berkaitan dengan rentang alamat ([addr]ess) IP
- `amass intel -active -addr 192.168.0.1-254` : gunakan metode pengintaian secara aktif
- `amass intel -whois -d nama_domain` : cari para domain pangkal yang berkaitan dengan suatu domain
- `amass intel -org nama_organisasi` : cari para pihak ASN yang berkaitan dengan suatu [org]anisasi
- `amass intel -asn nomor_asn` : cari daftar domain yang dipegang oleh suatu pihak Nomor Sistem Otonom (ASN) berdasarkan nomornya
- `amass intel -o berkas_output -whois -d nama_domain` : simpan hasil pencarian ke dalam suatu berkas teks
- `amass intel -list` : tampilkan daftar sumber pencarian data
