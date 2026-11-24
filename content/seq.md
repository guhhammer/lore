# seq

Output a sequence of numbers to `stdout`.

## Usage
- `seq 10` : print a sequence from 1 to 10
- `seq 10 20` : print a sequence from 10 to 20
- `seq 5 3 20` : print every 3rd number from 5 to 20
- `seq 10 -1 1` : print a reverse sequence from 10 to 1
- `seq -s " " 5 3 20` : separate the output with a space instead of a newline
- `seq -f "%04g" 5 3 20` : format output width to a minimum of 4 digits padding with zeros as necessary
- `seq -w 5 3 20` : print all numbers with the same width

## Categories
documents

## Links
- More information: https://www.gnu.org/software/coreutils/manual/html_node/seq-invocation.html
- tldr page: https://tldr.inbrowser.app/pages/common/seq
- Wikipedia: https://en.wikipedia.org/wiki/Seq_(Unix)
