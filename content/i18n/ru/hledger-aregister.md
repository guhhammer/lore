# hledger aregister

Показывать транзакции и текущий баланс одного счёта, по одной транзакции на строку.

## Usage
- `hledger areg assets:bank:checking` : показать транзакции и текущий баланс счёта `assets:bank:checking`
- `hledger areg savings` : показать транзакции и текущий баланс первого счёта, содержащего `savings` в имени
- `hledger areg checking -C -w 120` : показать подтверждённые транзакции по расчётному счёту с указанной шириной
- `hledger areg checking --forecast` : показать регистр расчётного счёта, включая транзакции из правил прогнозирования

## Links
- More information: https://hledger.org/hledger.html#aregister
