# touch

Dosya oluştur ve erişim/değiştirilme zamanlarını ayarla.

## Usage
- `touch dosya/yolu1 dosya/yolu2 ...` : belirtilen dosyaları oluştur veya zaten varsa zaman damgalarını güncelle
- `touch -c -a|-m dosya/yolu1 dosya/yolu2 ...` : dosyanın erişim veya değiştirilme zamanını şimdiki zamana ayarla ve dosya yoksa oluşturma
- `touch -c -t YYYYMMDDHHMM.SS dosya/yolu1 dosya/yolu2 ...` : dosyanın zamanını belirli bir değere ayarla ve dosya yoksa oluşturma
- `touch -c -r referans_dosyası/yolu dosya/yolu1 dosya/yolu2 ...` : dosyaların zaman damgasını referans dosyasının zaman damgasına ayarla ve dosya yoksa oluşturma
- `touch -d "last year|5 hours|next thursday|nov 14|..." dosya/yolu` : zaman damgasını bir dizgiyi ayrıştırarak ayarla
- `touch dosya/yolu{1..10}` : artan sayılarla birden fazla dosya oluştur
- `touch dosya/yolu{a..z}` : harf aralığıyla birden fazla dosya oluştur
