# pahole

Shows, manipulates data structure layout and pretty prints raw data.

## Usage
- `pahole [options] files` : the general form, as the manual gives it
- `pahole -C` : show just these classes
- `pahole -c` : set cacheline size to SIZE bytes
- `pahole --compile` : generate compileable code, with all definitions for all types, i.e
- `pahole --count=COUNT` : pretty print the first COUNT records from input

## Links
- Documentation: https://nakryiko.com/posts/bpf-portability-and-co-re/
