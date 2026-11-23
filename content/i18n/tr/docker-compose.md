# docker compose

Çoklu konteynerli Docker uygulamalarını çalıştırın ve yönetin.

## Usage
- `docker compose ps` : tüm konteynerleri listele
- `docker compose up -d` : mevcut dizinde bir `docker-compose.yml` dosyası çalıştırarak arkaplandaki tüm konteynerleri çalıştırın ve başlatın
- `docker compose up --build` : tüm konteynerleri çalıştırın ve gerekiyorsa yeniden oluşturun
- `docker compose -p proje Adı -f yoldan/dosyaya up` : tüm konteynerleri alternatif bir beste dosyasıyla başlatın
- `docker compose stop` : çalışan tüm konteynerleri durdurun
- `docker compose down --rmi all -v` : tüm konteynerleri, ağları, imgeleri ve alanları durdurun ve silin
- `docker compose logs -f` : tüm konteynerler için logları takip edin
- `docker compose logs -f konteyner_ismi` : belirtilmiş bir konteyner için logları takip edin

## Links
- More information: https://docs.docker.com/reference/cli/docker/compose/
