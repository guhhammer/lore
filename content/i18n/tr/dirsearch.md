# dirsearch

Ağ yolu tarayıcı.

## Usage
- `dirsearch -u url --extensions-list` : bir ağ sunucusunu yaygın eklentiler içeren yaygın yollar için tarayın
- `dirsearch -l örnek/url-listesi.txt -e php` : ağ sunucularını içeren bir listeyi `.php` eklentili yaygın yollar için tarayın
- `dirsearch -u url --extensions-list -w path/to/url-yol-listesi.txt` : bir ağ sunucusunu yaygın eklentiler içeren belirtilen yollar için tarayın
- `dirsearch -u url -e php --cookie cookie` : bir ağ sunucusunu çerez kullanarak tarayın
- `dirsearch -u url -e php -m HEAD` : bir ağ sunucusunu `HEAD` HTTP metodunu kullanarak tarayın
- `dirsearch -u url -e php --json-report örnek/rapor_dosyası.json` : bir ağ sunucusunu tarayın ve sonuçları bir `.json` dosyasına kaydedin

## Links
- More information: https://github.com/maurosoria/dirsearch#options
