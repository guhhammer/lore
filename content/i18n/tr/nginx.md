# nginx

`nginx` web sunucusu.

## Usage
- `nginx` : varsayılan konfigürasyon dosyasıyla sunucuyu başlat
- `nginx -c konfigürasyon_dosyası` : özel bir konfigürasyon dosyasıyla sunucuyu başlat
- `nginx -c konfigürasyon_dosyası -p göreceli/dosya/yolu/ön/eki` : konfigürasyon dosyasındaki her göreceli dosya yolu için bir ön ek ile sunucuyu başlat
- `nginx -t` : çalışan sunucuyu etkilemeden konfigürasyon dosyasını test et
- `nginx -s reload` : aksamasız bir sinyal göndererek konfigürasyonu tekrar yükle

## Links
- More information: https://nginx.org/docs/switches.html
