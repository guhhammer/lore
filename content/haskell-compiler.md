# haskell-compiler

The Glasgow Haskell Compiler.

## Usage
- `ghc [option|filename] ghci [option|filename]` : the general form, as the manual gives it
- `haskell-compiler -fasm` : use the native code generator
- `haskell-compiler -fbyte-code` : generate byte-code
- `haskell-compiler -fbyte-code-and-object-code` : generate object code and byte-code
- `haskell-compiler -fexpose-internal-symbols` : produce symbols for all functions, including internal functions

## Categories
development

## Links
- Documentation: https://downloads.haskell.org/ghc/latest/docs/users_guide.pdf
