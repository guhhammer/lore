# arp-scan

Kirim paket ARP menuju kumpulan alamat IP atau host untuk memindai suatu jaringan komputer lokal.

## Usage
- `arp-scan --localnet` : pindai jaringan lokal yang terhubung saat ini
- `arp-scan 192.168.1.1/24` : pindai suatu alamat IP dengan pengaturan bitmask khusus
- `arp-scan 172.0.0.0-127.0.0.31` : pindai suatu jaringan IP menggunakan rentang alamat tertentu
- `arp-scan 10.0.0.0:255.255.255.0` : pindai suatu jaringan IP menggunakan net mask khusus

## Links
- More information: https://github.com/royhills/arp-scan
