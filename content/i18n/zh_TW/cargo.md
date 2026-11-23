# cargo

管理 Rust 專案及其模組相依項（crates）。

## Usage
- `cargo search 搜尋關鍵詞` : 搜尋套件（crate）
- `cargo install 套件名` : 安裝二進位套件（crate）
- `cargo install --list` : 列出已安裝的二進位套件（crate）
- `cargo init --bin|lib 路徑/至/目錄` : 在指定目錄（或預設情況下在目前工作目錄）中建立一個新的二進位或函式庫 Rust 專案
- `cargo add 相依項` : 向目前目錄的 `Cargo.toml` 新增一個相依項
- `cargo b -r` : 使用 release 模式在目前目錄中組建 Rust 專案
- `cargo +nightly b` : 使用最新的編譯器在目前目錄中組建 Rust 專案（需要 `rustup`）
- `cargo b -j 執行緒數` : 使用特定數量的執行緒組建（預設為邏輯 CPU 的數量）

## Links
- More information: https://doc.rust-lang.org/stable/cargo/
