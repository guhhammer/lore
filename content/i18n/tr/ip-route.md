# ip route

IP yönlendirme tablosu yönetimi alt komutu.

## Usage
- `ip r` : yönlendirme tablosunu görüntüle
- `sudo ip r a default via ağ_geçidi_ip_adresi` : ağ geçidini kullanan öntanımlı bir yönlendirme ekle
- `sudo ip r a default dev eth0` : `eth0` arayüzünü kullanan öntanımlı bir yönlendirme ekle
- `sudo ip r a hedef_ip_adresi via ağ_geçidi_ip_adresi dev eth0` : statik bir yönlendirme ekle
- `sudo ip r d hedef_ip_adresi dev eth0` : statik bir yönlendirmeyi sil
- `sudo ip r change|replace hedef_ip_adresi via ağ_geçidi_ip_adresi dev eth0` : statik bir yönlendirmeyi değiştir
- `ip r g hedef_ip_adresi` : bir IP adresine ulaşmak için çekirdek tarafından hangi yönlendirmenin kullanılacağını göster

## Links
- More information: https://manned.org/ip-route
