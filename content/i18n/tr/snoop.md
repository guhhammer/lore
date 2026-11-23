# snoop

Ağ paketi inceleyici. SunOS'in `tcpdump` alternatifi.

## Usage
- `snoop -d e1000g0` : belirtilen ağ arayüzünde paketleri yakala
- `snoop -o dosyaismi` : yakalanan paketleri terminalde göstermek yerine bir dosyaya kaydet
- `snoop -V -i dosyaismi` : belirtilen dosyadan paketlerin ayrıntılı protokol katman özetini görüntüle
- `snoop to port port from host hostismi` : host isminden gelen ağ paketlerini yakala ve belirtilen port'a git
- `snoop -x0 -p4 ip_addresi_1 ip_addresi_2` : i̇ki IP adresi arasında takas edilen ağ paketleriini yakala ve hex değerlerini göster

## Links
- More information: https://www.unix.com/man-page/sunos/1m/snoop
