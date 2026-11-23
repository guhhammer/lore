# llvm-reduce-19

LLVM automatic testcase reducer.

## Usage
- `llvm-reduce [options] [input...]` : the general form, as the manual gives it
- `llvm-reduce-19 --abort-on-invalid-reduction` : abort if any reduction results in invalid IR
- `llvm-reduce-19 -j <uint>` : maximum number of threads to use to process chunks
- `llvm-reduce-19 --mtriple=<string>` : set the target triple
- `llvm-reduce-19 --skip-delta-passes=<string>` : delta passes to not run, separated by commas

## Links
- Documentation: https://llvm.org/
