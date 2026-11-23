# dig

DNS sunucularına sorgulama yapmak için kullanılan bir komuttur.

## Usage
- `dig +short example.com` : bir ana bilgisayar adıyla ilişkili IP adres(ler)ini sorgula (A kayıtları)
- `dig +noall +answer example.com` : belirli bir alan adı için ayrıntılı bir yanıt al (A kayıtları)
- `dig +short example.com A|MX|TXT|CNAME|NS` : belirli bir alan adıyla ilişkili belirli bir DNS kaydı türünü sorgula
- `dig +tls @1.1.1.1|8.8.8.8|9.9.9.9|... example.com` : sorgulanacak alternatif bir DNS sunucusu belirt ve isteğe bağlı olarak TLS üzerinden DNS (DoT) kullan
- `dig -x 8.8.8.8` : bir IP adresi için tersine DNS sorgulaması yap (PTR kaydı)
- `dig +nssearch example.com` : bölgenin yetkili ad sunucularını bul ve SOA kayıtlarını görüntüle
- `dig +trace example.com` : i̇teratif sorgular yap ve bir alan adını çözümlemek için izlenen yolun tamamını görüntüle
- `dig +tcp -p port @dns_sunucusu_ip example.com` : standart olmayan bir port üzerinden TCP protokolünü kullanarak bir DNS sunucusunu sorgula

## Links
- More information: https://manned.org/dig
