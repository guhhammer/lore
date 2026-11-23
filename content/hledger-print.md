# hledger print

Show full journal entries, representing transactions.

## Usage
- `hledger print` : show all transactions in the default journal file
- `hledger print -x --infer-costs` : show transactions, with any implied amounts or costs made explicit
- `hledger print -f path/to/2023.journal -f path/to/2024.journal -B` : show transactions from two specified files, with amounts converted to cost
- `hledger print cur:\\$ food not:groceries date:thismonth` : show `$` transactions this month in accounts whose name contains `food` but not `groceries`
- `hledger print amt:'>50' desc:'whole foods'` : show transactions of amount 50 or more, with `whole foods` in their description
- `hledger print -C --commodity '1000, EUR' --round hard` : show cleared transactions, with `EUR` amounts rounded and with decimal commas
- `hledger print -f path/to/foo.journal -o path/to/output_file.csv` : write transactions from `foo.journal` as a CSV file

## Categories
documents, system, files

## Links
- More information: https://hledger.org/hledger.html#print
- tldr page: https://tldr.inbrowser.app/pages/common/hledger-print
