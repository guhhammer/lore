# gitlab-runner

GitLab koşucuları için CLI aracı.

## Usage
- `sudo gitlab-runner register --url https://gitlab.ornek.com --registration-token token --name isim` : bir koşucuyu kayıt ettir
- `sudo gitlab-runner register --url https://gitlab.ornek.com --registration-token token --name isim --executor docker` : bir koşucuyu Docker çalıştırıcısıyla kayı ettir
- `sudo gitlab-runner unregister --name isim` : bir koşucunun kaydını geri al
- `sudo gitlab-runner status` : koşucu servisinin durumunu görüntüle
- `sudo gitlab-runner restart` : koşucu servisini yeniden başlat
- `sudo gitlab-runner verify` : kayıt edilen koşucuların GitLab'e bağlanabilme durumlarını kontrol et

## Links
- More information: https://docs.gitlab.com/runner/
