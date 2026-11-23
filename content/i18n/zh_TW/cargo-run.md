# cargo run

執行目前的 Cargo 套件。 注意：執行二進位的工作目錄將設定為目前工作目錄。

## Usage
- `cargo r` : 執行預設的二進位目標
- `cargo r --bin 名稱` : 執行指定的二進位檔案
- `cargo r --example 範例名` : 執行指定的範例
- `cargo r -F "功能1 功能2 ..."` : 啟用一系列以空格或逗號分隔的功能
- `cargo r --no-default-features` : 停用預設功能
- `cargo r --all-features` : 啟用所有可用的功能
- `cargo r --profile 名稱` : 使用指定的設定檔執行

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-run.html
