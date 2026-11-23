# go tool

Belirtilen bir Go aracını veya komutunu çalıştır. Bir Go komutunu tipik olarak hata ayıklamak için tek başına bir binary olarak çalıştır.

## Usage
- `go tool` : erişilebilir araçları sırala
- `go tool link örnek/konum/main.o` : go bağ aracını çalıştır
- `go tool -n komut argümanları` : çalıştırılacak komutu çalıştırmadan yazdır (`whereis`'e benzer)
- `go tool komut --help` : belirtilen araç için resmi dokümentasyonu göster

## Links
- More information: https://pkg.go.dev/cmd/go#hdr-Run_specified_go_tool
