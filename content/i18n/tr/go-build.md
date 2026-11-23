# go build

Go kaynaklarını derle.

## Usage
- `go build örnek/konum/main.go` : bir 'package main' dosyasını derle (çıktı uzantısız bir dosya ismi olacak)
- `go build -o örnek/konum/binary örnek/konum/kaynak.go` : çıktı dosya ismini belirterek derle
- `go build -o örnek/konum/binary örnek/konum/paket` : bir paket yarat
- `go build -race -o örnek/konum/çalıştırılabilir örnek/konum/ana_paket` : bir ana paketi veri yarış tanımlayıcısını etkinleştirerek çalıştırılabilir olarak derle

## Links
- More information: https://pkg.go.dev/cmd/go#hdr-Compile_packages_and_dependencies
