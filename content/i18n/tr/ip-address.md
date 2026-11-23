# ip address

IP adresi yönetimi alt komutu.

## Usage
- `ip a` : ağ arayüzlerini ve ilişkili IP adreslerini listele
- `ip a s up` : yalnızca etkin ağ arayüzlerini gösterecek şekilde filtrele
- `ip a s eth0` : belirli bir ağ arayüzü hakkındaki bilgileri görüntüle
- `sudo ip a a ip_adresi dev eth0` : bir ağ arayüzüne bir IP adresi ekle
- `sudo ip a d ip_adresi dev eth0` : bir ağ arayüzünden bir IP adresini kaldır
- `sudo ip a f eth0 scope global|host|link` : belirli bir kapsamdaki tüm IP adreslerini bir ağ arayüzünden sil

## Links
- More information: https://manned.org/ip-address
