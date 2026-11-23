# git check-ignore

Git yoksayma / dışlama (`.gitignore`) dosyalarını analiz et.

## Usage
- `git check-ignore örnek/dosya_veya_dizin` : bir dosya veya dizinin yoksayıldığı veya sayılmadığını kontrol et
- `git check-ignore örnek/dosya_veya_dizin1 örnek/dosya_veya_dizin2 ...` : birden fazla dosya veya dizinin yoksayıldığı veya sayılmadığını kontrol et
- `git <örnek/dosya_sırası check-ignore --stdin` : her bir satıra tekabül edecek şekilde `stdin`'den yolisimleri kullan
- `git check-ignore --no-index örnek/dosya_veya_dizin1 örnek/dosya_veya_dizin2 ...` : i̇ndeksi kontrol etme
- `git check-ignore -v örnek/dosya_veya_dizin1 örnek/dosya_veya_dizin2 ...` : her yol için eşleşen desene dair detayları dahil et

## Links
- More information: https://git-scm.com/docs/git-check-ignore
