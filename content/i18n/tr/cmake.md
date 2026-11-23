# cmake

Çok platformlu yapım otomasyon sistemi, yerel yapım sistemleri için tarifler oluşturur.

## Usage
- `cmake proje/dizininin/yolu` : bir proje dizinindeki `CMakeLists.txt` dosyasından, geçerli dizinde bir yapım tarifi oluştur
- `cmake --build yapım/dizininin/yolu` : belirtilen dizinde oluşturulmuş bir tarifi kullanarak yapım çıktılarını derle
- `cmake --install yapım/dizininin/yolu --strip` : yapım çıktılarını `/usr/local/` dizinine kur ve hata ayıklama sembollerini temizle
- `cmake proje/dizininin/yolu -D CMAKE_BUILD_TYPE=Release` : yapım türü CMake değişkeniyle `Release` olarak ayarlanmış bir yapım tarifi oluştur
- `cmake -G oluşturucu_adı proje/dizininin/yolu` : altta yatan yapım sistemi olarak `oluşturucu_adı` kullanan bir yapım tarifi oluştur
- `cmake --install yapım/dizininin/yolu --strip --prefix dizin/yolu` : yapım çıktılarını özel bir yol öneki kullanarak kur
- `cmake --build yapım/dizininin/yolu -t hedef_adı` : özel bir yapım hedefi çalıştır
- `cmake -h` : yardımı görüntüle

## Links
- More information: https://cmake.org/cmake/help/latest/manual/cmake.1.html
