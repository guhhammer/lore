# cargo

管理 Rust 项目及其模块依赖项（crates）。

## Usage
- `cargo search 搜索关键词` : 搜索包（crate）
- `cargo install 包名` : 安装二进制包（crate）
- `cargo install --list` : 列出已安装的二进制包（crate）
- `cargo init --bin|lib 路径/到/目录` : 在指定目录（或默认情况下在当前工作目录）中创建一个新的二进制或库 Rust 项目
- `cargo add 依赖项` : 向当前目录的 `Cargo.toml` 添加一个依赖项
- `cargo b -r` : 使用 release 模式在当前目录中构建 Rust 项目
- `cargo +nightly b` : 使用最新的编译器在当前目录中构建 Rust 项目（需要 `rustup`）
- `cargo b -j 线程数` : 使用特定数量的线程构建（默认为逻辑 CPU 的数量）

## Links
- More information: https://doc.rust-lang.org/stable/cargo/
