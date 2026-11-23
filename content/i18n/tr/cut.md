# cut

Standart girdiden (`stdin`) veya dosyalardan alanları kes.

## Usage
- `komut | cut -c 5` : satırların beşinci karakterini yazdır
- `cut -c 5-10 dosya/yolu` : belirtilen dosyadaki satırların beşinci ile onuncu arasındaki karakterlerini yazdır
- `cut -f 2,6 dosya/yolu` : bir dosyadaki satırları bir ayırıcıya göre alanlara böl ve ikinci ile altıncı alanları yazdır (varsayılan ayırıcı `TAB`'dır)
- `komut | cut -d "ayırıcı" -f 2-` : satırları belirtilen ayırıcıya göre böl ve ikinci alandan itibaren hepsini yazdır
- `komut | cut -d " " -f -3` : ayırıcı olarak boşluk kullan ve yalnızca ilk 3 alanı yazdır
- `komut | cut -d ":" -f 1 -s` : yalnızca ayırıcıyı içeren satırları yazdır
- `find . -print0 | cut -z -d "/" -f 2` : satır sonu olarak yeni satır yerine `NUL` kullanan satırların belirli alanlarını yazdır

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/cut-invocation.html
