# iptables

Linux çekirdek IPv4 güvenlik duvarının; tablolarını, zincirlerini ve kurallarını yapılandırın. IPv6 trafiğini ayarlamak için `ip6tables` kullanın.

## Usage
- `sudo iptables -vnL --line-numbers` : filtre tablosu için zincirleri, kuralları, paket/bayt sayaçlarını ve satır numaralarını görüntüleyin
- `sudo iptables -P zincir kural` : zincir politikası kuralını ayarlayın
- `sudo iptables -A zincir -s ip_address -j kural` : verilen kuralı belirtilen IP için zincir politikasına uygulayın
- `sudo iptables -A zincir -s ip_address -p tcp|udp|icmp|... --dport port -j kural` : protokol ve portu dikkate alarak verilen IP için zincir politikasına kural ekle
- `sudo iptables -t nat -A POSTROUTING -s 192.168.0.0/24 -j MASQUERADE` : `192.168.0.0/24` alt ağından gelen tüm trafiği ana bilgisayarın açık IP adresine yönlendirmek için bir NAT kuralı ekleyin
- `sudo iptables -D zincir kural_satır_numarası` : zincir kuralını silin

## Links
- More information: https://manned.org/iptables
