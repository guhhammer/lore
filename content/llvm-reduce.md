# llvm-reduce

LLVM automatic testcase reducer.

## Usage
- `llvm-reduce [options] [input...]` : the general form, as the manual gives it
- `llvm-reduce --abort-on-invalid-reduction` : abort if any reduction results in invalid IR
- `llvm-reduce -j <uint>` : maximum number of threads to use to process chunks
- `llvm-reduce --mtriple=<string>` : set the target triple
- `llvm-reduce --skip-delta-passes=<string>` : delta passes to not run, separated by commas

## Links
- Documentation: https://llvm.org/
