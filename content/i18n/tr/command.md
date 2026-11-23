# command

Kabuğun aynı adlı fonksiyonları, yerleşik komutları ve takma adları yoksayarak programı çalıştırmasını zorla.

## Usage
- `command ls` : aynı adlı bir takma ad olsa bile `ls` programını doğrudan çalıştır
- `command -p komut` : tüm standart yardımcı programları bulmayı garanti eden varsayılan bir `$PATH` (`/bin:/usr/bin:/sbin:/usr/sbin:/etc:/usr/etc`) kullanarak bir komutu bul ve çalıştır
- `command -v komut` : belirli bir komutun çalıştırılabilir dosyasının yolunu veya takma ad tanımını göster

## Links
- More information: https://www.gnu.org/software/bash/manual/bash.html#index-command
