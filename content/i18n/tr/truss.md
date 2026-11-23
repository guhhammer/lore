# truss

İzleme sistem çağrıları için sorun giderme aracı. SunOS'in strace alternatifi.

## Usage
- `truss -f program` : bir programı tüm alt işlemleriyle beraber çalıştırarak başlat
- `truss -p pid` : belirtilen işlemi onun PID değerini girerek izlemeye başla
- `truss -a -e program` : bir programı argümanları ve çevresel değerlerini göstererek başlar
- `truss -c -p pid` : her bir sistem çağrısı için zaman, çağrı ve hataları say ve program çıkışında bunların özetini bildir
- `truss -p pid -t system_çağrısı_ismi` : bir işlemi onun çıktısını sistem çağrısıyla süzerek izle

## Links
- More information: https://www.unix.com/man-page/sunos/1/truss
