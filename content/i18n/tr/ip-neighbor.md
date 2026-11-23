# ip neighbor

Komşu/ARP tablosu yönetimi IP alt komutu.

## Usage
- `ip n` : komşu/ARP tablosu girdilerini görüntüle
- `sudo ip n f dev eth0` : `eth0` aygıtının komşu tablosundaki girdileri kaldır
- `ip n g aranacak_ip dev eth0` : bir komşu araması gerçekleştir ve bir komşu girdisi döndür
- `sudo ip n add|delete ip_adresi lladdr mac_adresi dev eth0 nud reachable` : `eth0` arayüzüne komşu IP adresi için bir ARP girdisi ekle veya sil
- `sudo ip n change|replace ip_adresi lladdr yeni_mac_adresi dev eth0` : `eth0` arayüzünde komşu IP adresi için bir ARP girdisini değiştir

## Links
- More information: https://manned.org/ip-neighbour
