# rustup set

Изменять настройки `rustup`.

## Usage
- `rustup set default-host тройка_хоста` : установить тройку хоста по умолчанию
- `rustup set profile minimal|default` : установить профиль по умолчанию (`minimal` включает только `rustc`, `rust-std` и `cargo`, тогда как `default` добавляет `rust-docs`, `rustfmt` и `clippy`)
- `rustup set auto-self-update enable|disable|check-only` : указать, должен ли `rustup` обновлять себя при выполнении `rustup update`

## Links
- More information: https://rust-lang.github.io/rustup/
