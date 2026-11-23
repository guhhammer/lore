# cargo test

执行 Rust 包的单元测试和集成测试。

## Usage
- `cargo t 测试名称` : 仅运行包含特定字符串在其名称中的测试
- `cargo t -- --test-threads 数量` : 设置并行运行测试用例的数量
- `cargo t -r` : 在 release 模式下测试构建，启用优化
- `cargo t --workspace` : 测试工作区中的所有包
- `cargo t -p 包名` : 为特定包运行测试
- `cargo t -- --nocapture` : 运行测试时不隐藏测试执行的输出

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-test.html
