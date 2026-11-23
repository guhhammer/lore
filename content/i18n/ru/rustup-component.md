# rustup component

Управлять установленными компонентами набора инструментов. Без опции `--toolchain` `rustup` использует набор инструментов по умолчанию. Подробнее см. `rustup help toolchain`.

## Usage
- `rustup component add --toolchain набор_инструментов компонент` : добавить компонент в набор инструментов
- `rustup component remove --toolchain набор_инструментов компонент` : удалить компонент из набора инструментов
- `rustup component list --toolchain набор_инструментов` : показать установленные и доступные компоненты для набора инструментов
- `rustup component list --toolchain набор_инструментов --installed` : показать установленные компоненты для набора инструментов

## Links
- More information: https://rust-lang.github.io/rustup/
