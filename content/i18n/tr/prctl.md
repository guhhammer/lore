# prctl

Çalışan işlemlerin, görevlerin ve projelerin kaynak kontrollerini öğren veya belirle.

## Usage
- `prctl pid` : belirtilen işlemin limit ve izinlerini incele
- `prctl -P pid` : i̇şlem limit ve izinlerini makineye dayanıklı formatta incele
- `prctl -n process.max-file-descriptor pid` : çalışan işlem için belirtilen limiti öğren

## Links
- More information: https://www.unix.com/man-page/sunos/1/prctl
