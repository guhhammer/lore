# date

Sistem tarihini görüntüleyin veya ayarlayın.

## Usage
- `date +%c` : varsayılan yerel biçimi kullanarak geçerli tarihi görüntüleyin
- `date -u +%Y-%m-%dT%H:%M:%SZ` : geçerli tarihi UTC ve ISO 8601 formatında görüntüleyin
- `date +%s` : geçerli tarihi bir Unix zaman damgası olarak görüntüleyin (Unix zamanından bu yana geçen saniyeler)
- `date -d @1473305798` : unix zaman damgası olarak belirtilen bir tarihi varsayılan biçime dönüştürün
- `date -d "2018-09-01 00:00" +%s -u` : belirli bir tarihi Unix zaman damgası biçimine dönüştürün
- `date --rfc-3339 date|seconds|ns` : geçerli tarihi RFC-3339 biçiminde (`YYYY-AA-GG ss:dd:sn+ZD`) görüntüleyin ve duyarlılığını belirtin
- `sudo date 093023592021.59` : geçerli tarihi `AAGGssddYYYY.sn` biçimini kullanarak ayarlayın (`YYYY` ve `.sn` isteğe bağlıdır)
- `date +%V` : geçerli ISO hafta numarasını görüntüleyin

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/date-invocation.html
