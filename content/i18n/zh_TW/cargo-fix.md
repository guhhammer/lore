# cargo fix

自動修復 `rustc` 報告的 lint 警告。

## Usage
- `cargo fix --broken-code` : 即使已經有編譯器錯誤，也要修復程式碼
- `cargo fix --allow-dirty` : 即使工作目錄有更改，也要修復程式碼
- `cargo fix --edition` : 將一個套件遷移到下一個 Rust 版本
- `cargo fix --lib` : 修復套件的函式庫
- `cargo fix --test 名稱` : 修復指定的整合測試
- `cargo fix --workspace` : 修復工作區中的所有成員

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-fix.html
