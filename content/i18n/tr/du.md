# du

Disk kullanımı: dosyaların ve dizinlerin kapladığı alanı tahmin et ve özetle.

## Usage
- `du -b|k|m dizin/yolu` : bir dizinin ve tüm alt dizinlerinin boyutlarını verilen birimde (B/KiB/MiB) listele
- `du -h dizin/yolu` : bir dizinin ve tüm alt dizinlerinin boyutlarını okunabilir biçimde (yani uygun birimi otomatik seçerek) listele
- `du -sh dizin/yolu` : tek bir dizinin boyutunu okunabilir birimlerle göster
- `du -ah dizin/yolu` : bir dizinin ve içindeki tüm dosya ve dizinlerin boyutlarını okunabilir biçimde listele
- `du -h -d n dizin/yolu` : bir dizinin ve alt dizinlerinin boyutlarını, en fazla `n` düzey derinliğe kadar okunabilir biçimde listele
- `du -ch *.jpg` : mevcut dizindeki tüm `.jpg` dosyalarının boyutlarını okunabilir biçimde listele ve sonunda genel toplamı göster
- `du -ah -t 1G|1024M|1048576K .[^.]* *` : belirli bir eşik boyutunun üzerindeki tüm dosya ve dizinleri (gizli olanlar dahil) listele (alanı asıl neyin kapladığını bulmak için kullanışlıdır)

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/du-invocation.html
