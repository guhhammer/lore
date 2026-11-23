# go vet

Go kaynak kodunu kontrol et ve şüpheli yapıları bildir (örneğin Go kaynak dosyalarını tiftik et). Go vet komutu eğer sorun bulunduysa sıfır olmayan bir çıkış kodu yazdırır. Eğer herhangi bir sorun bulunmadıysa sıfır çıkış kodu yazdırılır.

## Usage
- `go vet` : mevcut dizindeki Go paketini kontrol et
- `go vet örnek/dosya_veya_dizin` : belirtilen yoldaki Go paketini kontrol et
- `go tool vet help` : go vet ile çalıştırılabilecek erişilebilir kontrolleri sırala
- `go tool vet help kontrol_ismi` : belirtilen bir kontrol için detayları ve bayrakları göster
- `go vet -c=N` : kontrolün sorun bulmasına sebep olan satırları artı N sayıda ek içeriği görüntüle
- `go vet -json` : analiz ve hataları JSON formatında çıkart

## Links
- More information: https://pkg.go.dev/cmd/vet
