# cargo clean

刪除 `target` 目錄中產生的組建產物。

## Usage
- `cargo clean` : 刪除整個 `target` 目錄
- `cargo clean --doc` : 刪除文件組建產物（`target/doc` 目錄）
- `cargo clean -r` : 刪除 release 模式的組建產物（`target/release` 目錄）
- `cargo clean --profile dev` : 刪除指定設定檔的目錄中的組建產物（在此例中為 `target/debug`）

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-clean.html
