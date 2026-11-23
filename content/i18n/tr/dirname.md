# dirname

Belirtilen dosya veya yolun ana dizinini hesaplar.

## Usage
- `dirname dosya_veya_dizine/giden/yol` : belirtilen yolun ana dizinini hesapla
- `dirname dosya_veya_dizine/giden/yol_1 dosya_veya_dizine/giden/yol_2 ...` : birden çok yolun ana dizinini hesapla
- `dirname -z dosya_veya_dizine/giden/yol_1 dosya_veya_dizine/giden/yol_2 ...` : komut çıktısını yeni satır yerine NUL karakteri ile sınırlandırma (`xargs` yazılımı ile kullanırken işe yarar)
