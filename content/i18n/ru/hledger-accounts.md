# hledger accounts

Выводить список имён счетов.

## Usage
- `hledger acc` : показать все счета, использованные или объявленные в файле журнала по умолчанию
- `hledger acc -u` : показать счета, использованные в транзакциях
- `hledger acc -d` : показать счета, объявленные с помощью директив account
- `hledger acc --undeclared --directives >>2024-accounts.journal` : добавить новые директивы account для использованных, но не объявленных счетов в журнал
- `hledger acc asset --types` : показать счета, содержащие `asset` в имени, и их объявленные/выведенные типы
- `hledger acc type:A` : показать счета типа `Asset`
- `hledger acc -t -2` : показать первые два уровня иерархии счетов

## Links
- More information: https://hledger.org/hledger.html#accounts
