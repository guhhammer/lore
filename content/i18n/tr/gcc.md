# gcc

C ve C++ kaynak dosyalarını ön işlemden geçir ve derle, ardından assembler ile nesne koduna çevirip birbirine bağla. GCC'nin (GNU Compiler Collection) bir parçasıdır.

## Usage
- `gcc yol/kaynak1.c yol/kaynak2.c ... -o yol/çalıştırılabilir_dosya` : birden fazla kaynak dosyayı çalıştırılabilir bir dosyaya derle
- `gcc yol/kaynak.c -Wall -o çalıştırılabilir_dosya` : tüm hataların ve uyarıların çıktıda gösterilmesini etkinleştir
- `gcc yol/kaynak.c -Wall -g -Og -o yol/çalıştırılabilir_dosya` : yaygın uyarıları göster, çıktıya hata ayıklama sembollerini ekle ve hata ayıklamayı etkilemeyecek şekilde optimize et
- `gcc yol/kaynak.c -o yol/çalıştırılabilir_dosya -Iyol/başlık -Lyol/kütüphane -lkütüphane_adı` : farklı bir yoldaki kütüphaneleri dahil et
- `gcc -S yol/kaynak.c` : kaynak kodunu Assembly komutlarına derle
- `gcc -c yol/kaynak.c` : kaynak kodunu bağlamadan bir nesne dosyasına derle
- `gcc yol/kaynak.c -O1|2|3|fast -o yol/çalıştırılabilir_dosya` : derlenen programı performans için optimize et
- `gcc --version` : sürümü görüntüle

## Links
- More information: https://gcc.gnu.org/onlinedocs/gcc/
