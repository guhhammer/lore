# llvm-cxxfilt

LLVM symbol name demangler.

## Usage
- `llvm-cxxfilt [options] [mangled names...]` : the general form, as the manual gives it
- `llvm-cxxfilt --format=<value>` : mangling scheme to assume
- `llvm-cxxfilt --help` : print a summary of command line options
- `llvm-cxxfilt --no-params` : do not demangle function parameters or return types
- `llvm-cxxfilt --no-strip-underscore` : do not strip a leading underscore

## Links
- Documentation: https://llvm.org/
