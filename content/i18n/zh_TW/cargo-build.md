# cargo build

編譯本地套件及其所有相依項。

## Usage
- `cargo b` : 在本地路徑中組建由 `Cargo.toml` 清單檔案定義的一個或多個套件
- `cargo b -r` : 以 release 模式組建，並進行最佳化
- `cargo b --locked` : 要求 `Cargo.lock` 檔案為最新版本
- `cargo b --workspace` : 組建工作區中的所有套件
- `cargo b -p 套件` : 組建特定的套件
- `cargo b --bin 名稱` : 僅組建指定的二進位檔案
- `cargo b --test 測試名稱` : 僅組建指定的測試目標

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-build.html
