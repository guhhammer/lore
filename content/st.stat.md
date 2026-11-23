# st.stat

Print basic descriptive statistics from input numbers.

## Usage
- `st path/to/file` : print count, min, max, sum, mean, and standard deviation for numbers in a file
- `cat path/to/file | st` : print statistics from `stdin`
- `st -s path/to/file` : print only the sum of the numbers
- `st -m path/to/file` : print only the mean of the numbers
- `st -sd path/to/file` : print only the standard deviation
- `st -to path/to/file` : transpose output (keys in one column, values in another)

## Categories
files, documents, monitoring

## Links
- More information: https://github.com/nferraz/st
- tldr page: https://tldr.inbrowser.app/pages/common/st.stat
