# mount

Tek bir dizindeki tüm dosya sistemlerine erişin.

## Usage
- `mount` : bağlı olan tüm dosya sistemlerini görüntüleyin
- `mount aygıt_dosyası/yolu hedef_dizin/yolu` : bir aygıtı bir dizine bağlayın
- `mount -m aygıt_dosyası/yolu hedef_dizin/yolu` : bir aygıtı dizine eğer o dizin yoksa oluşturarak bağlayın
- `mount -o uid=kullanıcı_id,gid=grup_id aygıt_dosyası/yolu hedef_dizin/yolu` : bir aygıtı bir dizine özel kullanıcı için bağlayın
- `mount -t iso9660 -o ro /dev/cdrom /cdrom` : bir CD-ROM aygıtını (dosya tipi ISO9660) `/cdrom`'a bağlayın (salt okunur)
- `mount -a` : `/etc/fstab`'da tanımlanmış olan tüm dosya sistemlerini bağlayın
- `mount /my_drive` : `/etc/fstab`'da bahsedilen özel bir dosya sistemini bağlayın (örneğin `/dev/sda1 /my_drive ext2 defaults 0 2`)
- `mount -B eski_dizin/yolu yeni_dizin/yolu` : bir dizini başka bir dizine bağlayın

## Links
- More information: https://manned.org/mount.8
