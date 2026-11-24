# gcov

Code coverage analysis and profiling tool that discovers untested parts of a program. Also displays a copy of source code annotated with execution frequencies of code segments.

## Usage
- `gcov path/to/file.cpp` : generate a coverage report named `file.cpp.gcov`
- `gcov -a path/to/file.cpp` : write individual execution counts for every basic block
- `gcov -b path/to/file.cpp` : write branch frequencies to the output file and print summary information to `stdout` as a percentage
- `gcov -c path/to/file.cpp` : write branch frequencies as the number of branches taken, rather than the percentage
- `gcov -n path/to/file.cpp` : do not create a `gcov` output file
- `gcov -f path/to/file.cpp` : write file level as well as function level summaries

## Categories
files, development

## Links
- More information: https://gcc.gnu.org/onlinedocs/gcc/Invoking-Gcov.html
- tldr page: https://tldr.inbrowser.app/pages/linux/gcov
- Wikipedia: https://en.wikipedia.org/wiki/Gcov
