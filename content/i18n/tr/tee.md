# tee

Standart girdiden (`stdin`) oku, `stdout`'a ve dosyalara (veya komutlara) yaz.

## Usage
- `echo "örnek" | tee dosya/yolu` : standart girdiyi (`stdin`) belirtilen dosyalara ve ayrıca `stdout`'a kopyala
- `echo "örnek" | tee -a dosya/yolu` : verilen dosyaların üzerine yazma, sonuna ekle
- `echo "örnek" | tee /dev/tty | xargs printf "[%s]"` : standart girdiyi (`stdin`) terminale yazdır ve ayrıca işlenmesi için başka bir programa aktar
- `echo "örnek" | tee >(xargs mkdir) >(wc -c)` : adı "örnek" olan bir dizin oluştur, "örnek" içindeki karakterleri say ve "örnek" metnini terminale yaz

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tee-invocation.html
