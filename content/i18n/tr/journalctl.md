# journalctl

systemd günlüğünü sorgulayın.

## Usage
- `journalctl -b -p 3` : kullanılan önyüklemeden 3 öncelik seviyesine sahip tüm mesajları(hataları) gösterin
- `journalctl --vacuum-time 2d` : 2 günden eski olan günlük kayıtlarını silin
- `journalctl -n n -f` : yeni mesajların sadece son 'n' satırlarını gösterin ve takip edin (geleneksel syslog için `tail -f` benzeri)
- `journalctl -u birim` : belirli bir birime ait tüm mesajları göster
- `journalctl _SYSTEMD_INVOCATION_ID=$(systemctl show --value --property=InvocationID birim)` : belirli bir birimin son başlatılmasından bu yana tüm günlük kayıtlarını göster
- `journalctl -S now|today|yesterday|tomorrow -U "YYYY-MM-DD HH:MM:SS"` : belirli bir zaman aralığındaki mesajları filtrele (zaman damgası veya "dün" gibi yer tutucular)
- `journalctl _PID=pid` : belirli bir işleme ait tüm mesajları göster
- `journalctl çalıştırabilir/dosya/dizin/yolu` : çalıştırılabilir bir uygulama dosyasından gelen tüm mesajları gösterin

## Links
- More information: https://www.freedesktop.org/software/systemd/man/latest/journalctl.html
