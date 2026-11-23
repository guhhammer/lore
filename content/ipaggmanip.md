# ipaggmanip

Manipulate aggregate statistics produced by `ipaggcreate`.

## Usage
- `ipaggmanip -p 16 path/to/file` : combine labels equal in their high-order bits
- `ipaggmanip --cut-smaller 100 --cull-labels 5 path/to/file` : remove labels with a count smaller than a given number of bytes and output a random sample of such labels
- `ipaggmanip -P path/to/file` : replace each label's count with 1 if it is non-zero

## Categories
files, monitoring

## Links
- More information: https://manned.org/ipaggmanip
- tldr page: https://tldr.inbrowser.app/pages/common/ipaggmanip
