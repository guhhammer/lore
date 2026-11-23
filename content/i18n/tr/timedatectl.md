# timedatectl

Sistem tarih ve zamanını ayarlayın.

## Usage
- `timedatectl` : mevcut sistem saatini kontrol edin
- `timedatectl set-time "yyyy-aa-gg SS:dd:ss"` : sistem yerel saatini doğrudan ayarlayın
- `timedatectl list-timezones` : ayarlanabilir zaman dilimlerini listeleyin
- `timedatectl set-timezone zaman dilimi` : sistem zaman dilimini ayarlayın
- `timedatectl set-ntp on` : ağ Zaman Protokolü (NTP) eşlemesini aktifleştirin
- `timedatectl set-local-rtc 1` : standart donanım zamanını yerel saatiniz olarak ayarlayın

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/timedatectl.html
