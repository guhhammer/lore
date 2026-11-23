# docker network

Docker ağları oluştur ve yönet.

## Usage
- `docker network ls` : docker daemon'daki tüm müsait ve düzenlenmiş ağları sırala
- `docker network create -d driver_name ağ_ismi` : kullanıcı tarafından belirtilmiş bir ağ oluştur
- `docker network inspect ağ_ismi` : boşluk ile ayrılmış bir ağ listesinin detaylı bilgisini görüntüle
- `docker network connect ağ_ismi konteyner_ismi|id` : bir konteyneri isim veya ID kullanarak bir ağa bağla
- `docker network disconnect ağ_ismi konteyner_ismi|id` : bir konteyneri bir ağdan çıkar
- `docker network prune` : tüm kullanılmayan (hiçbir konteyner tarafından belirtilmeyen) ağları sil
- `docker network rm ağ_ismi` : kullanılmayan ağların boşluk ile ayrılmış bir listesini sil

## Links
- More information: https://docs.docker.com/reference/cli/docker/network/
