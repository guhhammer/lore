# snoop

Pengendus paket jaringan. Perintah setara `tcpdump` untuk SunOS.

## Usage
- `snoop -d e1000g0` : tangkap paket pada antarmuka jaringan tertentu
- `snoop -o jalan/menuju/berkas` : simpan paket yang tertangkap pada sebuah berkas dari pada menampilkannya
- `snoop -V -i jalan/menuju/berkas` : tampilkan rangkuman lapisan protokol paket-paket dari sebuah berkas dengan rinci
- `snoop to port port from host nama_host` : tangkap paket jaringan yang datang dari sebuah nama host dan pergi ke port yang ditentukan
- `snoop -x0 -p4 ip1 ip2` : tangkap dan tampilkan sebuah hex-dump dari pertukaran paket jaringan di antara 2 alamat IP

## Links
- More information: https://www.unix.com/man-page/sunos/1m/snoop
