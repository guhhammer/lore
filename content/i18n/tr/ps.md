# ps

Çalışan süreçler hakkında bilgi.

## Usage
- `ps aux` : çalışan tüm süreçleri listele
- `ps auxww` : çalışan tüm süreçleri komut dizgisinin tamamıyla birlikte listele
- `ps aux | grep [d]izgi` : bir dizgiyle eşleşen bir süreci ara (köşeli parantezler `grep` komutunun kendisiyle eşleşmesini önler)
- `ps -u $(id -u) -F` : mevcut kullanıcının tüm süreçlerini ekstra ayrıntılı biçimde listele
- `ps -u $(id -u) f` : mevcut kullanıcının tüm süreçlerini ağaç olarak listele
- `ps -o ppid= -p süreç_kimliği` : bir sürecin üst süreç kimliğini (PPID) al
- `ps k size` : süreçleri bellek tüketimine göre sırala
- `ps -L -p süreç_kimliği` : belirli bir sürecin iş parçacıklarını göster

## Links
- More information: https://manned.org/ps
