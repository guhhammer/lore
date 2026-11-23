# docker container ls

Docker konteynerlerini sırala.

## Usage
- `docker ps` : halihazırda çalışan Docker konteynerlerini listele
- `docker ps -a` : tüm (durmuş veya çalışan) Docker konteynerlerini listele
- `docker ps -l` : en son oluşturulan (durmuş veya çalışan) konteynerleri listele
- `docker ps -f "name=isim"` : i̇simlerinde belirtilen dizeleri içeren konteynerleri filtrele
- `docker ps -f "ancestor=imge:tag"` : belirtilen imge ile akrabalık taşıyan konteynerleri filtrele
- `docker ps -f "exited=kod" -a` : konteynerleri çıkış durum koduna göre filtrele
- `docker ps -f "status=mevcut_durum"` : konteynerleri mevcut durumlarına (oluşturulma, çalışma, silinme, durma, çıkma ve ölme) göre sırala
- `docker ps -f "volume=örnek/dizin" --format "table \{\{.ID\}\}\t\{\{.Image\}\}\t\{\{.Names\}\}\t\{\{.Mounts\}\}"` : belirtilmiş bir hacmi gömen veya belirtilmiş bir yola gömülmüş hacmi içeren konteynerleri filtrele

## Links
- More information: https://docs.docker.com/reference/cli/docker/container/ls/
