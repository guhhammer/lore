# docker inspect

Docker objelerinde bulunan düşük seviye bilgiyi gösterir.

## Usage
- `docker inspect konteyner|imge|id` : bir konteyner, imge veya hacim ile ilgili bilgiyi ismini veya ID'sini girerek görüntüle
- `docker inspect -f '\{\{range.NetworkSettings.Networks\}\}\{\{.IPAddress\}\}\{\{end\}\}' konteyner` : bir konteynerin IP adresini görüntüle
- `docker inspect -f '\{\{.LogPath\}\}' konteyner` : konteynerin log dosyasının yolunu görüntüle
- `docker inspect -f '\{\{.Config.Image\}\}' konteyner` : konteynerin imge ismini görüntüle
- `docker inspect -f '\{\{json .Config\}\}' konteyner` : konfigürasyon bilgisini JSON olarak görüntüle
- `docker inspect -f '\{\{range $p, $conf := .NetworkSettings.Ports\}\} \{\{$p\}\} -> \{\{(index $conf 0).HostPort\}\} \{\{end\}\}' konteyner` : tüm port limanlayıcıları görüntüle
- `docker inspect` : yardımı görüntüle

## Links
- More information: https://docs.docker.com/reference/cli/docker/inspect/
