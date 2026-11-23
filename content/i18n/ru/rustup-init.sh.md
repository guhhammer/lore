# rustup-init.sh

Скрипт для установки `rustup` и набора инструментов Rust.

## Usage
- `curl https://sh.rustup.rs -sSf | sh -s` : скачать и запустить `rustup-init` для установки `rustup` и набора инструментов Rust по умолчанию
- `curl https://sh.rustup.rs -sSf | sh -s -- аргументы` : скачать и запустить `rustup-init` с передачей аргументов
- `rustup-init.sh --target целевая_платформа --component компонент` : запустить `rustup-init` с указанием дополнительных компонентов или целевых платформ
- `rustup-init.sh --default-toolchain набор_инструментов` : запустить `rustup-init` с указанием набора инструментов по умолчанию
- `rustup-init.sh --default-toolchain none` : запустить `rustup-init` без установки набора инструментов
- `rustup-init.sh --profile minimal|default|complete` : запустить `rustup-init` с указанием профиля установки
- `rustup-init.sh -y` : запустить `rustup-init` без запроса подтверждения

## Links
- More information: https://forge.rust-lang.org/infra/other-installation-methods.html#rustup
