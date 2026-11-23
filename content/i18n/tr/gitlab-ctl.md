# gitlab-ctl

Çok amaçlı GitLab yönetim CLI aracı.

## Usage
- `sudo gitlab-ctl status` : tüm servislerin durumunu görüntüle
- `sudo gitlab-ctl status nginx` : belirtilen servisin durumunu görüntüle
- `sudo gitlab-ctl restart` : tüm servisleri yeniden başlat
- `sudo gitlab-ctl restart nginx` : belirtilen servisi yeniden başlat
- `sudo gitlab-ctl tail` : tüm servislerin kaydını görüntüle ve `<Ctrl c>` basılana kadar okumaya devam et
- `sudo gitlab-ctl tail nginx` : belirtilen servisin kaydını görüntüle

## Links
- More information: https://docs.gitlab.com/omnibus/maintenance/
