# ganache-cli

Command-line version of Ganache, your personal blockchain for Ethereum development.

## Usage
- `ganache-cli` : run Ganache
- `ganache-cli --accounts=number_of_accounts` : run Ganache with a specific number of accounts
- `ganache-cli --secure` : run Ganache and lock available accounts by default
- `ganache-cli --secure --unlock "account_private_key1" --unlock "account_private_key2"` : run Ganache server and unlock specific accounts
- `ganache-cli --account="account_private_key,account_balance"` : run Ganache with a specific account and balance
- `ganache-cli --defaultBalanceEther=default_balance` : run Ganache with accounts with a default balance
- `ganache-cli --verbose` : run Ganache and log all requests to `stdout`

## Categories
text

## Links
- More information: https://archive.trufflesuite.com/ganache/
- tldr page: https://tldr.inbrowser.app/pages/common/ganache-cli
