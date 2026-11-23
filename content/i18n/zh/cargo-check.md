# cargo check

检查本地软件包及其所有依赖项是否有错误。

## Usage
- `cargo c` : 检查当前软件包
- `cargo c --tests` : 检查所有测试
- `cargo c --test integration_test1` : 检查 `tests/integration_test1.rs` 中的集成测试
- `cargo c -F feature1,feature2` : 使用 `feature1` 和 `feature2` 功能检查当前软件包
- `cargo c --no-default-features` : 禁用默认功能后检测当前软件包

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-check.html
