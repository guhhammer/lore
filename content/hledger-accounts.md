# hledger accounts

List account names.

## Usage
- `hledger acc` : show all accounts used or declared in the default journal file
- `hledger acc -u` : show accounts used by transactions
- `hledger acc -d` : show accounts declared with account directives
- `hledger acc --undeclared --directives >>2024-accounts.journal` : add new account directives, for accounts used but not declared, to the journal
- `hledger acc asset --types` : show accounts with `asset` in their name, and their declared/inferred types
- `hledger acc type:A` : show accounts of the `Asset` type
- `hledger acc -t -2` : show the first two levels of the accounts hierarchy

## Categories
users, system

## Links
- More information: https://hledger.org/hledger.html#accounts
- tldr page: https://tldr.inbrowser.app/pages/common/hledger-accounts
