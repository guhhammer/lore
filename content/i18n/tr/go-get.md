# go get

Bir bağımlılık paketi ekle veya eski GOPATH modunda paket indir.

## Usage
- `go get ornek.com/pkg` : `go.mod`'a modül modunda (module-mode) belirtilen bir paket ekle veya paketi GOPATH modunda indir
- `go get ornek.com/pkg@v1.2.3` : paketi module-aware modunda belirtilen sürümde düzenle
- `go get ornek.com/pkg@none` : belirtilen paketi sil
