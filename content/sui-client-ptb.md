# sui client ptb

Create, sign, and execute programmable transaction blocks.

## Usage
- `sui client ptb --move-call p::m::f "<type>" args` : call a Move function from a package and module
- `sui client ptb --make-move-vec "<u64>" "[1000,2000]"` : make a Move vector with two elements of type u64
- `sui client ptb --split-coins gas "[1000]" --assign new_coins --transfer-objects "[new_coins]" @address` : split a gas coin and transfer it to address
- `sui client ptb --transfer-objects "[object_id]" @address` : transfer an object to an address
- `sui client ptb --move-call sui::tx_context::sender --assign sender --publish "." --assign upgrade_cap --transfer-objects "[upgrade_cap]" sender` : publish a Move package and transfer the upgrade capability to sender

## Categories
files, packages

## Links
- More information: https://docs.sui.io/references/cli/ptb
- tldr page: https://tldr.inbrowser.app/pages/common/sui-client-ptb
