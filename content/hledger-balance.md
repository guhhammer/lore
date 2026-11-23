# hledger balance

A flexible, general purpose "summing" report that shows accounts with some kind of numeric data. This can be balance changes per period, end balances, budget performance, unrealized capital gains, etc.

## Usage
- `hledger bal` : show the balance change in all accounts from all postings over all time
- `hledger bal expenses -t -2` : show the balance change in accounts whose name contains `expenses`, as a tree, summarizing the top two levels only
- `hledger bal expenses -M -T -A -S --budget` : show expenses each month, and their totals and averages, sorted by total; and their monthly budget goals
- `hledger bal type:X -MTAS --budget -t -2 --no-elide` : show monthly totals, averages, and budget goals for accounts matching the `Expense` type, as a two level tree without squashing boring accounts
- `hledger bal -H -p 'quarterly in 2024' assets liabilities` : show end balances (including from postings before the start date), quarterly in 2024, in accounts whose name contains `assets` or `liabilities`
- `hledger bal -HQ date:2024 type:AL -ES -3` : show quarterly end balances in 2024 for asset and liability accounts, including zero balances, sorted by total, and summarized to three levels
- `hledger bal -HVQ assets:investments` : show investment assets' market Value in base currency at the end of each quarter
- `hledger bal --gain -Q assets:investments not:cryptocurrency` : show unrealized capital gains/losses from market price changes in each quarter, for non-cryptocurrency investment assets

## Categories
monitoring

## Links
- More information: https://hledger.org/hledger.html#balance
- tldr page: https://tldr.inbrowser.app/pages/common/hledger-balance
