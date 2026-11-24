# add up numbers in a file

Sum a column of numbers, from a file or a command's output.

## Steps
- `awk '{s+=$1} END {print s}' numbers.txt` : the sum of the first column
- `awk -F, '{s+=$3} END {print s}' data.csv` : the sum of the third column of a CSV
- `paste -sd+ numbers.txt | bc` : one number a line, added with bc
- `awk '{s+=$1; n++} END {print s/n}' numbers.txt` : the average

## See also
- awk
- paste
- bc

## Categories
text
