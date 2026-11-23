# ip rule

IP yönlendirme politikası veri tabanı yönetimi.

## Usage
- `ip ru` : yönlendirme politikasını göster
- `sudo ip ru a from 192.168.178.2/32` : paket kaynak adreslerine dayalı yeni bir kural ekle
- `sudo ip ru a to 192.168.178.2/32` : paket hedef adreslerine dayalı yeni bir kural ekle
- `sudo ip ru d from 192.168.178.2/32` : paket kaynak adreslerine dayalı bir kuralı sil
- `sudo ip ru d to 192.168.178.2/32` : paket hedef adreslerine dayalı bir kuralı sil
- `sudo ip ru f` : silinen tüm kuralları temizle
- `ip ru s >ip_kuralları.dat/dosyasının/yolu` : tüm kuralları bir dosyaya kaydet
- `sudo ip ru r <ip_kuralları.dat/dosyasının/yolu` : tüm kuralları bir dosyadan geri yükle

## Links
- More information: https://manned.org/ip-rule
