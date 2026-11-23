# ip link

Ağ arayüzlerini yönet.

## Usage
- `ip l` : tüm ağ arayüzleriyle ilgili bilgileri göster
- `ip l sh ethN` : belirli bir ağ arayüzüyle ilgili bilgileri göster
- `sudo ip l s ethN up|down` : bir ağ arayüzünü etkinleştir veya devre dışı bırak
- `sudo ip l s ethN al "LAN Arayüzü"` : bir ağ arayüzüne anlamlı bir ad ver
- `sudo ip l s ethN a ff:ff:ff:ff:ff:ff` : bir ağ arayüzünün MAC adresini değiştir
- `sudo ip l s ethN mtu 9000` : jumbo çerçeveleri kullanması için bir ağ arayüzünün MTU boyutunu değiştir

## Links
- More information: https://manned.org/ip-link
