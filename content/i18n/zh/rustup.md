# rustup

安装、管理和更新 Rust 工具链。 一些子命令，如 `toolchain`、`target`、`update` 等有自己的使用文档。

## Usage
- `rustup install nightly` : 为你的系统安装 nightly 工具链
- `rustup default nightly` : 将默认工具链切换为 nightly 以便 `cargo` 和 `rustc` 命令使用它
- `rustup override set nightly` : 在当前项目中使用 nightly 工具链，但保持全局设置不变
- `rustup update` : 更新所有工具链
- `rustup show` : 列出已安装的工具链
- `rustup run 工具链 cargo build` : 使用指定工具链运行 `cargo build`
- `rustup doc` : 在默认网页浏览器中打开本地 Rust 文档

## Links
- More information: https://rust-lang.github.io/rustup/
