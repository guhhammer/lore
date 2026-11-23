# hledger add

Записывать новые транзакции с помощью интерактивных подсказок в консоли.

## Usage
- `hledger add` : записать новые транзакции, сохраняя в файл журнала по умолчанию
- `hledger add -f путь/к/2024.journal -f путь/к/2023.journal` : добавить транзакции в `2024.journal`, загрузив также `2023.journal` для автодополнения
- `hledger add today 'best buy' expenses:supplies '$20'` : предоставить ответы на первые четыре запроса
- `hledger add -h` : показать параметры и документацию `add` с помощью `$PAGER`
- `hledger help add` : показать документацию `add` с помощью `info` или `man`, если доступно

## Links
- More information: https://hledger.org/hledger.html#add
