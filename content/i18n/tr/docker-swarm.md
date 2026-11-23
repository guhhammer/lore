# docker swarm

Bir konteyner orkestrasyon aracı.

## Usage
- `docker swarm init` : bir bataklık dizisi oluştur
- `docker swarm join-token işçi|yönetici` : bir yönetici veya işçiye takılmak için token göster
- `docker swarm join --token token manager_node_url:2377` : diziye yeni bir düğüm ekle
- `docker swarm leave` : bir işçiyi bataklıktan sil (işçi düğümünün içinde çalıştır)
- `docker swarm ca` : mevcut CA sertifikasını PEM formatında görüntüle
- `docker swarm ca --rotate` : mevcut CA sertifikasını döndür ve yeni sertifikayı görüntüle
- `docker swarm update --cert-expiry saathdakikamsaniyes` : düğüm sertifikaları için geçerli periyodu değiştir

## Links
- More information: https://docs.docker.com/engine/swarm/
