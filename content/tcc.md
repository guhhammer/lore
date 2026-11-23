# tcc

A tiny C compiler that can run C source files as scripts and otherwise has command-line options similar to `gcc`.

## Usage
- `tcc -o executable_name path/to/file1.c path/to/file2.c` : compile and link 2 source files to generate an executable
- `tcc -run path/to/source_file.c arguments` : directly run an input file like a script and pass arguments to it
- `#!/path/to/tcc -run` : interpret C source files with a shebang inside the file

## Categories
files, development, text

## Links
- More information: https://bellard.org/tcc/tcc-doc.html
- tldr page: https://tldr.inbrowser.app/pages/common/tcc
