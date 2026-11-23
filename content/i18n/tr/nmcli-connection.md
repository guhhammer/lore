# nmcli connection

NetworkManager ile bağlantı yönetimi.

## Usage
- `nmcli c` : tüm NetworkManager bağlantılarını listele (ad, UUID, tür ve aygıtı gösterir)
- `nmcli c u uuid` : UUID belirterek bağlantıyı etkinleştir
- `nmcli c d uuid` : bağlantıyı devre dışı bırak
- `nmcli c a ifname arayüz_adı type ethernet ipv4.method auto ipv6.method auto` : IPv4 ve IPv6 otomatik olarak yapılandırılan bir bağlantı oluştur
- `nmcli c a ifname arayüz_adı type ethernet ip6 2001:db8::2/64 gw6 2001:db8::1 ipv6.dns 2001:db8::1 ipv4.method ignore` : statik bir yalnızca IPv6 bağlantısı oluştur
- `nmcli c a ifname arayüz_adı type ethernet ip4 10.0.0.7/8 gw4 10.0.0.1 ipv4.dns 10.0.0.1 ipv6.method ignore` : statik bir yalnızca IPv4 bağlantısı oluştur
- `nmcli c i type openvpn file vpn_yapılandırması.ovpn/dosyasının/yolu` : bir OVPN dosyasından OpenVPN kullanan bir VPN bağlantısı oluştur
