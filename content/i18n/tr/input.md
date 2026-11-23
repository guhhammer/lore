# input

Olay kodlarını ve dokunmatik ekran mimiklerini bir Android cihazına yolla. Bu komut yalnızca `adb shell` ile kullanılabilir.

## Usage
- `input keyevent etkinlik_kodu` : bir Android cihazına tek karakter için etkinlik kodu gönder
- `input text "yazı"` : bir Android cihazına yazı gönder (`%s` boşlukları temsil eder)
- `input tap x_poz y_poz` : bir Android cihazına tek dokunuş gönder
- `input swipe x_başlangıç y_başlangıç x_son y_son ms_süre` : bir Android cihazına kaydırma mimiği gönder
- `input swipe x_poz y_poz x_poz y_poz ms_süre` : bir Android cihazına kaydırma mimiği kullanarak uzun dokunuş gönder
