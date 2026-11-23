# rustup set

修改 `rustup` 的配置。

## Usage
- `rustup set default-host 目标平台` : 设置默认目标平台
- `rustup set profile minimal|default` : 设置默认安装配置（`minimal` 只包含 `rustc`、`rust-std` 和 `cargo`，而 `default` 还会安装 `rust-docs`、`rustfmt` 和 `clippy`）
- `rustup set auto-self-update enable|disable|check-only` : 设置在执行 `rustup update` 时是否自动更新 `rustup` 本身

## Links
- More information: https://rust-lang.github.io/rustup/
