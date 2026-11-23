# hledger incomestatement

Show revenue inflows and expense outflows during the report period. Amounts are shown with normal positive sign, as in conventional financial statements.

## Usage
- `hledger is` : show revenues and expenses (changes in `Revenue` and `Expense` accounts)
- `hledger is -M` : show revenues and expenses each month
- `hledger is -MTAS -2` : show monthly revenues/expenses/totals, largest first, summarized to 2 levels
- `hledger is -MTAS -2 -o is.html` : show monthly revenues/expenses/totals summarized to 2 levels, and generate HTML output in `is.html`

## Links
- More information: https://hledger.org/hledger.html#incomestatement
- tldr page: https://tldr.inbrowser.app/pages/common/hledger-incomestatement
