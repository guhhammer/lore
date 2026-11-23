# dumpsys

Android sistem servisleri ile ilgili bilgi sağla. Bu komut yalnızca `adb shell` ile kullanılabilir.

## Usage
- `dumpsys` : tüm sistem servisleri için tanısal bir çıktı al
- `dumpsys servis` : belirtilen sistem servisi için tanısal bir çıktı al
- `dumpsys -l` : `dumpsys` komutunun hakkında bilgi verebileceği tüm servisleri sırala
- `dumpsys servis -h` : bir servis için servise özel argümanları sırala
- `dumpsys --skip servis` : tanı çıktısından belirtilen servisi çıkart
- `dumpsys -t saniye` : saniye bazında bir zaman aşımı süresi belirle (varsayılan 10 saniyedir)

## Links
- More information: https://developer.android.com/tools/dumpsys
