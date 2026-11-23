# fakedata

Generate fake data using a large variety of generators.

## Usage
- `fakedata --generators` : list all valid generators
- `fakedata generator1 generator2 ...` : generate data using one or more generators
- `fakedata -f csv|tab|sql generator` : generate data with a specific output format
- `fakedata -l n generator` : generate a given number of data items (defaults to 10)
- `echo "\{\{Generator\}\}" | fakedata` : generate data using a custom output template (the first letter of generator names must be capitalized)

## Links
- More information: https://github.com/lucapette/fakedata
- tldr page: https://tldr.inbrowser.app/pages/common/fakedata
