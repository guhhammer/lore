# chmod

Bir dosyanın veya dizinin erişim izinlerini değiştir.

## Usage
- `chmod u+x dosya/yolu` : dosyanın sahibi olan kullanıcıya dosyayı çalıştırma izni ver
- `chmod u+rw dosya_veya_dizin/yolu` : kullanıcıya bir dosyayı/dizini okuma ve yazma izni ver
- `chmod g-x,o-x dosya/yolu` : gruptan ve diğerlerinden çalıştırma iznini kaldır
- `chmod a+rx dosya/yolu` : tüm kullanıcılara okuma ve çalıştırma izni ver
- `chmod o=g dosya/yolu` : diğerlerine (dosya sahibinin grubunda olmayanlara) grupla aynı izinleri ver
- `chmod o= dosya/yolu` : diğerlerinin tüm izinlerini kaldır
- `chmod 644 dosya/yolu` : i̇zinleri sekizlik gösterimle ayarla
- `chmod -R a+rX dizin/yolu` : dosyalar için tüm kullanıcılara özyinelemeli olarak okuma izni ver. Ayrıca en az bir çalıştırma izni olan dosyalara ve tüm alt dizinlere çalıştırma izni ver
