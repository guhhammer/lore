# dc

An arbitrary precision calculator. Uses reverse polish notation (RPN).

## Usage
- `dc` : start an interactive session
- `dc path/to/script.dc` : execute a script
- `dc -e '10 k 5 3 / p'` : calculate an expression with the specified scale
- `dc -e '4 5 * 17 - p'` : calculate 4 times 5 (4 5 *), subtract 17 (17 -), and print the output
- `dc -e '7 k 5 _3 / p'` : specify the number of decimal places to 7 (7 k), calculate 5 divided by -3 (5 _3 /) and print
- `dc -e '100 k 5 v 1 + 2 / p'` : calculate the golden ratio, phi: set number of decimal places to 100 (100 k), square root of 5 (5 v) plus 1 (1 +), divided by 2 (2 /), and print result

## See also
- bc
- qalc

## Categories
documents

## Links
- More information: https://www.gnu.org/software/bc/manual/dc-1.05/html_mono/dc.html
- tldr page: https://tldr.inbrowser.app/pages/common/dc
