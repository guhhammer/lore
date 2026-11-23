# roll

Roll a user-defined dice sequence.

## Usage
- `roll 3d` : roll 3 6-sided dice and sum the results
- `roll d8 + 3` : roll 1 8-sided die, add 3, and sum the results
- `roll 4d6h3` : roll 4 6-sided dice, keep the 3 highest results, and sum the results
- `roll --verbose 2{2d12}` : roll 2 12-sided dice 2 times and show every roll
- `roll "2d20>10"` : roll 2 20-sided dice until the result is bigger than 10
- `roll --sum-series 3{2d5}` : roll 2 5-sided dice 3 times and show the total sum

## Categories
users

## Links
- More information: https://manned.org/roll
- tldr page: https://tldr.inbrowser.app/pages/common/roll
