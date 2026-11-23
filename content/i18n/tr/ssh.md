# ssh

Secure Shell, uzak sistemlere güvenli bağlantı kurmak için kullanılan bir protokoldür. Uzak sunucuda oturum açmak veya komut çalıştırmak için kullanılabilir.

## Usage
- `ssh kullanici@uzak_sunucu` : uzak bir sunucuya bağlan
- `ssh kullanici@uzak_sunucu -i anahtar/dosyasi/yolu` : belirli bir kimlik dosyası (özel anahtar) ile uzak sunucuya bağlan
- `ssh kullanici@10.0.0.1 -p 2222` : `10.0.0.1` IP adresli uzak sunucuya belirli bir port ile bağlan (Not: `10.0.0.1` kısaltılarak `10.1` yazılabilir)
- `ssh kullanici@uzak_sunucu -t komut komut_argumanlari` : uzak sunucuda etkileşimli komut çalıştırmak için tty tahsisi ile bağlan
- `ssh kullanici@uzak_sunucu -D 1080` : SSH tünelleme: Dinamik port yönlendirme (`localhost:1080` üzerinde SOCKS proxy)
- `ssh kullanici@uzak_sunucu -L 9999:example.org:80 -N -T` : SSH tünelleme: Belirli bir portu yönlendir (`localhost:9999`'dan `example.org:80`'e), sahte tty tahsisini ve uzak komut çalıştırmayı devre dışı bırak
- `ssh kullanici@uzak_sunucu -J kullanici@atlama_sunucusu` : SSH atlama: Bir atlama sunucusu üzerinden uzak sunucuya bağlan (birden fazla atlama virgülle ayrılabilir)
- `<Enter><~><.>` : takılmış bir oturumu kapat

## Links
- More information: https://man.openbsd.org/ssh
