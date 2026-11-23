# find

Özyinelemeli olarak bir dizin ağacı altında dosyaları ve dizinleri bulur.

## Usage
- `find kök_dizin -name '*.uzantı'` : dosyaları uzantılarına göre bul
- `find kök_dizin -path '*/yol/*/*.uzantı' -or -name '*desen*'` : birden fazla yol/ad deseniyle eşleşen dosyaları bul
- `find kök_dizin -type d -iname '*lib*'` : verilen bir adla eşleşen dizinleri, büyük/küçük harfe duyarsız modda bul
- `find kök_dizin -name '*.py' -not -path '*/site-packages/*'` : belirli yolları hariç tutarak, verilen bir desenle eşleşen dosyaları bul
- `find kök_dizin -maxdepth 1 -size +500k -size -10M` : özyineleme derinliğini "1" ile sınırlayarak, belirli bir boyut aralığıyla eşleşen dosyaları bul
- `find kök_dizin -name '*.uzantı' -exec wc -l {} \;` : her dosya için bir komut çalıştır (dosya adına erişmek için komut içinde `{}` kullanın)
- `find kök_dizin -daystart -mtime -1 -exec tar -cvf archive.tar {} \+` : bugün değiştirilmiş tüm dosyaları bul ve sonuçları tek bir komuta argüman olarak aktar
- `find kök_dizin -type f|d -empty -delete -print` : boş dosyaları veya dizinleri ara ve adlarını yazdırarak sil

## Links
- More information: https://manned.org/find
