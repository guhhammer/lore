# cargo

Rust projelerini ve modül bağımlılıklarını (crate'leri) yönet. `build` gibi bazı alt komutların kendi kullanım dokümantasyonu vardır.

## Usage
- `cargo search arama_ifadesi` : crate ara
- `cargo install crate_adı` : çalıştırılabilir bir crate yükle
- `cargo install --list` : yüklü çalıştırılabilir crate'leri listele
- `cargo init --bin|lib yol/dizin` : belirtilen dizinde (varsayılan olarak geçerli çalışma dizininde) yeni bir çalıştırılabilir ya da kütüphane Rust projesi oluştur
- `cargo add bağımlılık` : geçerli dizindeki `Cargo.toml` dosyasına bir bağımlılık ekle
- `cargo b -r` : geçerli dizindeki Rust projesini release profiliyle derle
- `cargo r` : derlenmiş çalıştırılabilir dosyayı çalıştır (henüz derlenmemişse önce derler)
- `cargo +nightly b` : geçerli dizindeki Rust projesini nightly derleyiciyle derle (`rustup` gerektirir)

## Links
- More information: https://doc.rust-lang.org/stable/cargo/
