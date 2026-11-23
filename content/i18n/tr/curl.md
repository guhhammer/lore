# curl

Bir sunucuya veya sunucudan veri aktarır. HTTP, HTTPS, FTP, SCP gibi birçok protokolü destekler.

## Usage
- `curl https://example.com` : bir HTTP GET isteği yap ve içeriği `stdout`'a yaz
- `curl -L -D - https://example.com` : HTTP GET isteği yap, `3xx` yönlendirmelerini takip et ve yanıt başlıklarıyla içeriği `stdout`'a yaz
- `curl -O https://example.com/dosya.zip` : bir dosyayı indir ve URL'deki dosya adıyla kaydet
- `curl -X POST -d 'isim=ali' http://example.com/form` : form ile kodlanmış veri gönder (`application/x-www-form-urlencoded` türünde POST isteği). `stdin`'den okumak için `--data @dosya_adı` veya `--data @'-'` kullan
- `curl -k -x http://127.0.0.1:8080 -H 'Authorization: Bearer token' -X GET|PUT|POST|DELETE|PATCH|... https://example.com` : ekstra başlık, özel HTTP yöntemi ve proxy (BurpSuite gibi) ile istek gönder, güvensiz sertifikaları yoksay
- `curl -d '{"isim":"ali"}' -H 'Content-Type: application/json' http://example.com/users/1234` : JSON formatında veri gönder ve uygun Content-Type başlığını belirt
- `curl -E istemci.pem --key anahtar.pem -k https://example.com` : i̇stemci sertifikası ve özel anahtar ile istek gönder, sertifika doğrulamasını atla
- `curl -v --resolve example.com:80:127.0.0.1 http://example.com` : bir alan adını özel bir IP adresine çözümle, ayrıntılı çıktı ile (DNS çözümlemesi için `/etc/hosts` dosyasını düzenlemeye benzer)

## Links
- More information: https://curl.se/docs/manpage.html
