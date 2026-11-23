# hledger

A robust, friendly plain text accounting app.

## Usage
- `hledger add` : record new transactions interactively, saving to the default journal file
- `hledger import path/to/bank.csv` : import new transactions from `bank.csv`, using `bank.csv.rules` to convert
- `hledger print -f path/to/prices-2024.journal -f path/to/prices-2023.journal` : print all transactions, reading from multiple specified journal files
- `hledger accounts -t --types` : show all accounts, as a hierarchy, and their types
- `hledger bs -E -t --no-elide` : show asset and liability account balances, including zeros, hierarchically
- `hledger is -M -T -A --sort -2` : show monthly incomes/expenses/totals, largest first, summarized to 2 levels
- `hledger areg assets:bank:checking` : show the `assets:bank:checking` account's transactions and running balance
- `hledger print assets:cash | hledger -f - -I aregister expenses:food` : show the amount spent on food from the `assets:cash` account

## See also
- hledger-ui
- hledger-web

## Categories
text, files, system

## Links
- More information: https://hledger.org/hledger.html
- tldr page: https://tldr.inbrowser.app/pages/common/hledger
