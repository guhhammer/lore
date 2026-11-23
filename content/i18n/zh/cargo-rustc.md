# cargo rustc

编译一个 Rust 软件包。类似于 `cargo build`，但您可以向编译器传递额外的选项。 查看 `rustc --help` 获取所有可用选项。

## Usage
- `cargo rustc -- rustc_options` : 构建软件包并向 `rustc` 传递选项
- `cargo rustc -r` : 在 release 模式下构建，启用优化
- `cargo rustc -r -- -C target-cpu=native` : 使用针对当前 CPU 的特定架构优化编译
- `cargo rustc -- -C opt-level=1|2|3` : 使用速度优化编译
- `cargo rustc -- -C opt-level=s|z` : 使用体积（size）优化编译（`z` 也会关闭循环向量化）
- `cargo rustc --lib -- -D unsafe-code` : 检查您的软件包是否使用了不安全的代码
- `cargo rustc -p 软件包` : 构建特定的软件包
- `cargo rustc --bin 名称` : 仅构建指定的二进制文件

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-rustc.html
