# hledger balancesheet

Show the end balances in asset and liability accounts. Amounts are shown with normal positive sign, as in conventional financial statements.

## Usage
- `hledger bs` : show the current balances in `Asset` and `Liability` accounts, excluding zeros
- `hledger bs type:C` : show just the liquid assets (`Cash` account type)
- `hledger bs -E -t` : include accounts with zero balances, and show the account hierarchy
- `hledger bs -M` : show the balances at the end of each month
- `hledger bs -M -V` : show the balances' market value in home currency at the end of each month
- `hledger bs -Qt -2` : show quarterly balances, with just the top two levels of account hierarchy
- `hledger bs -Qt -2 -o bs.html` : show quarterly balances with the top two levels of account hierarchy and generate HTML output in `bs.html`

## Categories
users

## Links
- More information: https://hledger.org/hledger.html#balancesheet
- tldr page: https://tldr.inbrowser.app/pages/common/hledger-balancesheet
