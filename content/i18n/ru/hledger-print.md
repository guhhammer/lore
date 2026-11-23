# hledger print

Показывать полные записи журнала, представляющие транзакции.

## Usage
- `hledger print` : показать все транзакции в файле журнала по умолчанию
- `hledger print -x --infer-costs` : показать транзакции, раскрыв все подразумеваемые суммы или стоимости
- `hledger print -f путь/к/2023.journal -f путь/к/2024.journal -B` : показать транзакции из двух указанных файлов с суммами, конвертированными в стоимость
- `hledger print cur:\\$ food not:groceries date:thismonth` : показать транзакции в `$` за текущий месяц по счетам, содержащим `food`, но не `groceries`
- `hledger print amt:'>50' desc:'whole foods'` : показать транзакции на сумму 50 или более с `whole foods` в описании
- `hledger print -C --commodity '1000, EUR' --round hard` : показать подтверждённые транзакции с суммами в `EUR`, округлёнными и с десятичными запятыми
- `hledger print -f путь/к/foo.journal -o путь/к/выходному_файлу.csv` : записать транзакции из `foo.journal` в CSV-файл

## Links
- More information: https://hledger.org/hledger.html#print
