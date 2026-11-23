# cargo run

运行当前的 Cargo 软件包。 注意: 执行二进制的工作目录将设置为当前工作目录。

## Usage
- `cargo r` : 运行默认的二进制目标
- `cargo r --bin 名称` : 运行指定的二进制文件
- `cargo r --example 示例名` : 运行指定的示例
- `cargo r -F "功能1 功能2 ..."` : 激活一系列以空格或逗号分隔的功能
- `cargo r --no-default-features` : 禁用默认功能
- `cargo r --all-features` : 激活所有可用的功能
- `cargo r --profile 名称` : 使用指定的配置文件运行

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-run.html
