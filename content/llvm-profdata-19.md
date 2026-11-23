# llvm-profdata-19

Profile data tool.

## Usage
- `llvm-profdata command [args...]` : the general form, as the manual gives it
- `llvm-profdata-19 --output=<output>` : specify the output file name
- `llvm-profdata-19 --sample` : specify that the input profile is a sample-based profile
- `llvm-profdata-19 --num-threads=<N>` : use N threads to perform profile merging
- `llvm-profdata-19 --compress-all-sections=[true|false]` : compress all sections when writing the profile

## Links
- Documentation: https://llvm.org/
