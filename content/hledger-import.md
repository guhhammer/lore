# hledger import

Import new transactions from one or more data files to the main journal.

## Usage
- `hledger import path/to/bank.csv` : import new transactions from `bank.csv`, using `bank.csv.rules` to convert
- `hledger import path/to/bank1.csv path/to/bank2.csv --dry-run` : simulate importing from these two files, showing what would be imported
- `hledger import --rules-file common.rules *.csv` : import new transactions from all CSV files, using the same rules for all
- `watchexec -- hledger -f path/to/bank.csv print` : show conversion errors or results while editing `bank.csv.rules`
- `hledger import --catchup path/to/bank.csv` : mark `bank.csv`'s current data as seen, as if already imported
- `rm -f .latest.bank.csv` : mark `bank.csv` as all new, as if not yet imported

## Categories
text, files, system

## Links
- More information: https://hledger.org/hledger.html#import
- tldr page: https://tldr.inbrowser.app/pages/common/hledger-import
