# node

Sunucu tarafı JavaScript platformu (Node.js).

## Usage
- `node dosya/yolu` : javaScript dosyası çalıştır
- `node` : REPL (interactive shell) başlat
- `node --watch dosya/yolu` : bir içe aktarılan dosya değiştirildiği zaman belirtilen dosyayı yeniden başlat (Node.js sürüm 18.11+ gerektirir)
- `node -e "kod"` : javaScript kodunu argüman olarak geçerek değerlendir
- `node -p "process.versions"` : sonucu değerlendir ve yazdır, Node.js bağımlılıklarının sürümlerini yazdırmak için kullanışlıdır
- `node --no-lazy --inspect-brk dosya/yolu` : denetleyiciyi etkinleştirerek, kaynak kodu tamamen ayrıştırıldıktan sonra bir hata ayıklayıcı bağlanana kadar yürütmeyi duraklatır

## Links
- More information: https://nodejs.org/docs/latest/api/cli.html#options
