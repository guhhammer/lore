# go test

Go paketlerini test et (dosyalar `_test.go` ifadesiyle bitmeli).

## Usage
- `go test` : mevcut dizinde bulunan paketleri test et
- `go test -v` : mevcut dizindeki paketleri ayrıntılı şekilde test et
- `go test -v ./...` : mevcut dizindeki ve tüm alt dizinlerdeki paketleri test et (`...` ifadesine dikkat)
- `go test -v -bench .` : mevcut dzindeki paketleri test et ve tüm kalite testlerini çalıştır
- `go test -v -bench . -benchtime 50s` : mevcut dizindeki paketleri test et ve 50 saniye içinde tüm kalite testlerini çalıştır
- `go test -cover` : paketleri kapsamlı bir analiz ile test et

## Links
- More information: https://pkg.go.dev/cmd/go#hdr-Testing_flags
