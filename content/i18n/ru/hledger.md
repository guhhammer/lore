# hledger

Надёжное и удобное приложение для учёта финансов в текстовом формате.

## Usage
- `hledger add` : записать новые транзакции в интерактивном режиме, сохраняя в файл журнала по умолчанию
- `hledger import путь/к/bank.csv` : импортировать новые транзакции из `bank.csv`, используя `bank.csv.rules` для преобразования
- `hledger print -f путь/к/prices-2024.journal -f путь/к/prices-2023.journal` : вывести все транзакции, читая из нескольких указанных файлов журнала
- `hledger accounts -t --types` : показать все счета в виде иерархии и их типы
- `hledger bs -E -t --no-elide` : показать балансы счетов активов и обязательств, включая нулевые, в виде иерархии
- `hledger is -M -T -A --sort -2` : показать ежемесячные доходы/расходы/итоги, по убыванию, с группировкой до 2-го уровня
- `hledger areg assets:bank:checking` : показать транзакции и текущий баланс счёта `assets:bank:checking`
- `hledger print assets:cash | hledger -f - -I aregister expenses:food` : показать сумму расходов на питание со счёта `assets:cash`

## Links
- More information: https://hledger.org/hledger.html
