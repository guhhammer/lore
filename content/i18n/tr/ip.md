# ip

Yönlendirmeyi, aygıtları, kural yönlendirmesini ve tünelleri görüntüle / değiştir. `address` gibi bazı alt komutların kendi kullanım belgeleri vardır.

## Usage
- `ip a` : arayüzlerin bilgilerini ayrıntılı bir şekilde listele
- `ip -br a` : arayüzlerin ağ katmanı bilgilerini kısa bir şekilde listele
- `ip -br l` : arayüzlerin bağlantı katmanı bilgilerini kısa bir şekilde listele
- `ip r` : yönlendirme tablosunu görüntüle
- `ip n` : komşuları (ARP tablosunu) göster
- `sudo ip l s arayüz up|down` : bir arayüzü etkinleştir/devre dışı bırak
- `sudo ip a add|delete ip_address/maske dev arayüz` : bir arayüze IP adresi ekle/sil
- `sudo ip r a default via ip_address dev arayüz` : öntanımlı yönlendirme ekle

## Links
- More information: https://manned.org/ip.8
