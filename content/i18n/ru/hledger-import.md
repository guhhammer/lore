# hledger import

Импортировать новые транзакции из одного или нескольких файлов данных в основной журнал.

## Usage
- `hledger import путь/к/bank.csv` : импортировать новые транзакции из `bank.csv`, используя `bank.csv.rules` для преобразования
- `hledger import путь/к/bank1.csv путь/к/bank2.csv --dry-run` : сымитировать импорт из этих двух файлов, показав, что будет импортировано
- `hledger import --rules-file common.rules *.csv` : импортировать новые транзакции из всех CSV-файлов, используя одни и те же правила для всех
- `watchexec -- hledger -f путь/к/bank.csv print` : показать ошибки преобразования или результаты при редактировании `bank.csv.rules`
- `hledger import --catchup путь/к/bank.csv` : пометить текущие данные `bank.csv` как просмотренные, как если бы они уже были импортированы
- `rm -f .latest.bank.csv` : пометить `bank.csv` как полностью новый, как если бы он ещё не был импортирован

## Links
- More information: https://hledger.org/hledger.html#import
