# ocamlfind

The findlib package manager for OCaml. Simplifies linking executables with external libraries.

## Usage
- `ocamlfind ocamlopt -package package1,package2,... -linkpkg -o path/to/executable path/to/source.ml` : compile a source file to a native binary and link with packages
- `ocamlfind ocamlc -package package1,package2,... -linkpkg -o path/to/executable path/to/source.ml` : compile a source file to a bytecode binary and link with packages
- `ocamlfind -toolchain cross-toolchain ocamlopt -o path/to/executable path/to/source.ml` : cross-compile for a different platform

## Categories
files, packages, development

## Links
- More information: https://manned.org/ocamlfind
- tldr page: https://tldr.inbrowser.app/pages/common/ocamlfind
