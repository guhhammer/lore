# lsusb

USB girişleri ve girişlere bağlı olan cihazlar hakkında bilgi görüntüleyin.

## Usage
- `lsusb` : mevcut tüm USB aygıtlarını listeleyin
- `lsusb -t` : USB aygıtlarını ağaç hiyerarşisi görünümünde listeleyin
- `sudo lsusb -v` : USB aygıtları hakkında ayrıntılı bilgileri görüntüleyin
- `sudo lsusb -v -s giriş:cihaz numarası` : bir USB aygıtı hakkında ayrıntılı bilgileri görüntüleyin
- `lsusb -d satıcı:cihaz` : sadece belirli satıcı ve cihaz ID'sine sahip aygıtları listeleyin

## Links
- More information: https://manned.org/lsusb
