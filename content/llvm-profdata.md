# llvm-profdata

Profile data tool.

## Usage
- `llvm-profdata command [args...]` : the general form, as the manual gives it
- `llvm-profdata --output=<output>` : specify the output file name
- `llvm-profdata --sample` : specify that the input profile is a sample-based profile
- `llvm-profdata --num-threads=<N>` : use N threads to perform profile merging
- `llvm-profdata --compress-all-sections=[true|false]` : compress all sections when writing the profile

## Links
- Documentation: https://llvm.org/
