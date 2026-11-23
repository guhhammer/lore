# cargo build

编译本地软件包及其所有依赖项。

## Usage
- `cargo b` : 在本地路径中构建由 `Cargo.toml` 清单文件定义的一个或多个软件包
- `cargo b -r` : 以 release 模式构建，并进行优化
- `cargo b --locked` : 要求 `Cargo.lock` 文件为最新版本
- `cargo b --workspace` : 构建工作区中的所有软件包
- `cargo b -p 软件包` : 构建特定的软件包
- `cargo b --bin 名称` : 仅构建指定的二进制文件
- `cargo b --test 测试名称` : 仅构建指定的测试目标

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-build.html
