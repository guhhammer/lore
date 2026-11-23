# docker system

Docker verilerini yönet ve sistem bilgisi görüntüle.

## Usage
- `docker system` : yardım göster
- `docker system df` : docker disk kullanımını göster
- `docker system df -v` : disk kullanımı üzerine detaylı bilgi göster
- `docker system prune` : kullanılmayan veriyi sil
- `docker system prune --filter "until=saathdakikam"` : kullanılmayan ve geçmişte birden çok kez oluşturulan veriyi sil
- `docker system events` : docker deamon'dan tam-zamanlı eylemleri görüntüle
- `docker system events -f 'type=container' --format '\{\{json .\}\}'` : geçerli JSON satırları olarak yayınlanan konteynerleden tam-zamanlı eylemleri göster
- `docker system info` : sistem bilgisi göster

## Links
- More information: https://docs.docker.com/reference/cli/docker/system/
