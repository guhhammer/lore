# docker secret

Docker swarm sırlarını yönet.

## Usage
- `komut | docker secret create sır_ismi -` : `stdin`'den yeni bir sır yarat
- `docker secret create sır_ismi örnek/dosya` : bir dosyadan yeni sır oluşturun
- `docker secret ls` : tüm sırları sırala
- `docker secret inspect --pretty sır_ismi1 sır_ismi2 ...` : bir veya daha fazla sırra dair detaylı bilgiyi insan dostu bir formatta göster
- `docker secret rm sır_ismi1 sır_ismi2 ...` : bir veya daha fazla sırrı sil

## Links
- More information: https://docs.docker.com/reference/cli/docker/secret/
