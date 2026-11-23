# tail

Bir dosyanın son kısmını göster.

## Usage
- `tail dosya/yolu` : bir dosyanın son 10 satırını göster
- `tail dosya/yolu1 dosya/yolu2 ...` : birden fazla dosyanın son 10 satırını göster
- `tail -5 dosya/yolu` : bir dosyanın son 5 satırını göster
- `tail -n +sayı dosya/yolu` : bir dosyayı belirli bir satır numarasından itibaren yazdır
- `tail -c sayı dosya/yolu` : verilen dosyanın sonundan belirli sayıda bayt yazdır
- `tail -f dosya/yolu` : verilen dosyanın son satırlarını yazdır ve `<Ctrl c>` tuşlarına basılana kadar okumaya devam et
- `tail -F dosya/yolu` : dosya erişilemez olsa bile `<Ctrl c>` tuşlarına basılana kadar okumaya devam et
- `tail -n sayı -s saniye -f dosya/yolu` : bir dosyanın son `sayı` satırını göster ve `saniye` saniyede bir yenile

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/tail-invocation.html
