# atool

Çeşitli türlerdeki dosya arşivlerini yöneten bir betik. `atool`, harici arşivleme programlarını kullanır ancak arşivleri listelemek, çıkartmak, oluşturmak ve yönetmek için tutarlı bir komut satırı arayüzü sunar.

## Usage
- `atool -l arşiv/dosyasının/yolu.zip` : bir arşivdeki dosyaları listele
- `atool -x arşiv.tar.gz` : bir arşivi çıkart (gerekirse güvenli bir şekilde alt dizin oluşturur)
- `atool -X çıktı/dizininin/yolu arşiv.rar` : bir arşivi belirli bir dizine çıkart
- `atool -c arşiv.tar arşivdeki/dosyanın/yolu.txt` : bir arşivdeki belirli bir dosyanın içeriğini `stdout`'a yazdır (`cat` gibi)
- `atool -a yeni_arşiv.zip dosya1/yolu dosya2/yolu ...` : belirtilen dosya ve/veya dizinlerden yeni bir arşiv oluştur
- `atool -l -p büyük_arşiv.tar.bz2` : bir arşivdeki dosyaları listele ve çıktıyı bir sayfalayıcıdan geçir
- `atool -x -e arşiv1.zip arşiv2.tar.gz *.rar` : birden fazla arşivi aynı anda çıkart (gerekirse her birini kendi alt dizinine)
- `atool -r eski_arşiv.tar.gz yeni_arşiv.tar.7z` : bir arşivi bir biçimden başka bir biçime yeniden paketle (örneğin `.tar.gz` biçiminden `.tar.7z` biçimine)

## Links
- More information: https://manned.org/atool
