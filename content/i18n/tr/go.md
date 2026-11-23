# go

Go kaynak kodunu yönetmeye yarayan bir araç. `build` gibi bazı alt komutların kendı kullanım dokümentasyonları mevcut.

## Usage
- `go get paket_yolu` : i̇çe aktarım yolunda belirtilen şekilde bir paketi indir ve yükle
- `go run dosya.go` : bir kaynak dosyasını derle ve çalıştır (bir `main` paketine sahip olmalı)
- `go build -o çalıştırılabilir dosya.go` : bir kaynak dosyasını belirtilen çalıştırılabilir dosyaya derle
- `go build` : mevcut dizinde bulunan paketi derle
- `go test` : mevcut paket için tüm test durumlarını çalıştır (bahsi geçen dosyalar `_test.go` ifadesi ile bitmeli)
- `go install` : mevcut paketi derle ve indir
- `go mod init modül_ismi` : mevcut diizinde yeni bir modül başlat

## Links
- More information: https://pkg.go.dev/cmd/go
