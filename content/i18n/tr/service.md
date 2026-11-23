# service

Başlatma betiklerini çalıştırarak hizmetleri/servisleri yönetin. Tam betik yolu atlanmalıdır (`/etc/init.d/` varsayılır).

## Usage
- `service --status-all` : tüm servisleri ve durumlarını görüntüleyin
- `service servis_adı start|stop|restart|reload` : bir servisi Başlatın/Durdurun/Yeniden başlatın/Yeniden yükleyin (başlat/durdur her zaman mevcut olmalıdır)
- `service servis_adı --full-restart` : tam yeniden başlatma yapın (betiği başlat ve durdur seçenekleriyle iki kez çalıştırır)
- `service servis_adı status` : servisin şuanki durumunu görüntüleyin

## Links
- More information: https://manned.org/service
