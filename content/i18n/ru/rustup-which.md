# rustup which

Показывать путь к исполняемому файлу команды, управляемой `rustup`. Аналог `which`, но выполняет поиск в наборе инструментов Rust вместо `$PATH`.

## Usage
- `rustup which команда` : показать путь к исполняемому файлу в наборе инструментов по умолчанию
- `rustup which --toolchain набор_инструментов команда` : показать путь к исполняемому файлу в указанном наборе инструментов (подробнее см. `rustup help toolchain`)

## Links
- More information: https://rust-lang.github.io/rustup/
