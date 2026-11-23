# cargo clippy

一系列 lint 工具，用於捕獲常見錯誤並改進 Rust 程式碼。

## Usage
- `cargo clippy` : 對目前目錄中的程式碼執行檢查
- `cargo clippy --locked` : 要求 `Cargo.lock` 檔案是最新的
- `cargo clippy --workspace` : 對工作區中的所有套件執行檢查
- `cargo clippy --package 套件` : 對某個套件執行檢查
- `cargo clippy -- -W clippy::lint 組` : 執行特定 lint 組的檢查（參見 <https://rust-lang.github.io/rust-clippy/stable/index.html#?groups=cargo,complexity,correctness,deprecated,nursery,pedantic,perf,restriction,style,suspicious>）
- `cargo clippy -- -D warnings` : 將警告視為錯誤
- `cargo clippy -- -A warnings` : 執行檢查並忽略警告
- `cargo clippy --fix` : 自動套用 Clippy 的建議

## Links
- More information: https://github.com/rust-lang/rust-clippy
