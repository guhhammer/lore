# git credential

Kullanıcı kimlik bilgilerini kurtar ve sakla.

## Usage
- `echo "url=http://example.com" | git credential fill` : kimlik bilgilerini, kullanıcı ismi ve parolayı konfigürasyon dosyası aracılığıyla kurtararak göster
- `echo "url=http://example.com" | git credential approve` : kimlik bilgilerini sonra kullanma amacıyla saklamak için bütün yapılandırılmış kimlik yardımcılarına gönder
- `echo "url=http://example.com" | git credential reject` : belirtilen kimlik bilgisini bütün yapılandırılmış kimlik yardımcılarından temizle

## Links
- More information: https://git-scm.com/docs/git-credential
