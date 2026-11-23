# ncat

Bir ağ üzerinden veriyi okuyun, yazın, yeniden yönlendirin veya şifreleyin. `netcat`/`nc` adlı benzer bir yardımcı programın alternatifi.

## Usage
- `ncat -l port >dosya/yolu` : belirtilen porttaki girdiyi dinle ve belirtilen dosyaya yaz
- `ncat -lk port` : çoklu bağlantıları kabul et ve ncat'i bağlantılar kapandıktan sonra da açık tut
- `ncat <dosya/yolu bilgisayar port` : belirtilen dosyanın çıktısını al ve belirtilen bilgisayardaki belirtilen porta yaz
- `ncat --ssl -k -l port` : trafik içeriğinin tespit edilmesini önleyerek şifreli bir kanalda çoklu bağlantıları kabul edin
- `ncat --ssl bilgisayar port` : açık `ncat` bağlantısını SSL üzerinden bağlayın
- `ncat -w saniye -vz bilgisayar port` : belirli bir bağlantı noktasındaki porta zaman aşımıyla bağlanılabilirliği kontrol edin

## Links
- More information: https://nmap.org/ncat/guide/index.html
