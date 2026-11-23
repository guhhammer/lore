# hledger aregister

Show the transactions and running balances in one account, with each transaction on one line.

## Usage
- `hledger areg assets:bank:checking` : show transactions and running balance in the `assets:bank:checking` account
- `hledger areg savings` : show transactions and running balance in the first account whose name contains `savings`
- `hledger areg checking -C -w 120` : show the checking account's cleared transactions, with a specified width
- `hledger areg checking --forecast` : show the checking register, including transactions from forecast rules

## Categories
users, text

## Links
- More information: https://hledger.org/hledger.html#aregister
- tldr page: https://tldr.inbrowser.app/pages/common/hledger-aregister
