# go fmt

Go kaynak dosyalarını formatla. Değiştirilen dosya isimlerini yazdırır.

## Usage
- `go fmt` : mevcut dizindeki Go kaynak dosyalarını formatla
- `go fmt örnek/konum/paket` : belirtilen Go paketini içe aktarım yolunda formatla (`$GOPATH/src`)
- `go fmt ./...` : paketi mevcut dizinde ve tüm öbür alt dizinlerde formatla (`...` ifadesine dikkat)
- `go fmt -n` : hiçbir şeyi düzenlemeden format komutlarının ne yapacağını yazdır
- `go fmt -x` : komut çalışırken arkaplanda hangi komutların çalıştığını yazdır

## Links
- More information: https://pkg.go.dev/cmd/go#hdr-Gofmt__reformat__package_sources
