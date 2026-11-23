# ab

Apache HTTP sunucusu kıyaslama aracı.

## Usage
- `ab -n 100 url` : belirli bir URL'ye 100 HTTP GET isteği gönder
- `ab -n 100 -c 10 url` : bir URL'ye, 10'luk eşzamanlı partiler halinde 100 HTTP GET isteği gönder
- `ab -n 100 -T application/json -p yol/dosya.json url` : bir dosyadan JSON verisi kullanarak bir URL'ye 100 HTTP POST isteği gönder
- `ab -k url` : HTTP keep-Alive kullan, yani tek bir HTTP oturumu içinde birden fazla istek gerçekleştir
- `ab -t 60 url` : kıyaslama için harcanacak maksimum süreyi (timeout) saniye cinsinden ayarla (varsayılan 30)
- `ab -e yol/dosya.csv` : sonuçları bir CSV dosyasına yaz

## Links
- More information: https://httpd.apache.org/docs/current/programs/ab.html
