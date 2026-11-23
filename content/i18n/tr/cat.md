# cat

Dosyaları yazdır ve birleştir.

## Usage
- `cat dosya/yolu` : bir dosyanın içeriğini `stdout`'a yazdır
- `cat dosya/yolu1 dosya/yolu2 ... >çıktı/dosyası/yolu` : birkaç dosyayı bir çıktı dosyasında birleştir
- `cat dosya/yolu1 dosya/yolu2 ... >>çıktı/dosyası/yolu` : birkaç dosyayı bir çıktı dosyasına ekle
- `cat -u /dev/tty12 >/dev/tty13` : bir dosyanın içeriğini arabelleğe almadan bir çıktı dosyasına kopyala
- `cat - >dosya/yolu` : `stdin`'i bir dosyaya yaz

## Links
- More information: https://manned.org/cat.1posix
