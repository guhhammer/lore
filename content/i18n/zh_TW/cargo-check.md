# cargo check

檢查本地套件及其所有相依項是否有錯誤。

## Usage
- `cargo c` : 檢查當前套件
- `cargo c --tests` : 檢查所有測試
- `cargo c --test integration_test1` : 檢查 `tests/integration_test1.rs` 中的整合測試
- `cargo c -F feature1,feature2` : 使用 `feature1` 和 `feature2` 功能檢查當前套件
- `cargo c --no-default-features` : 禁用預設功能後檢測當前套件

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-check.html
