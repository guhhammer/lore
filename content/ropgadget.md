# ropgadget

Find ROP gadgets in binary files.

## Usage
- `ROPgadget --binary path/to/binary` : list gadgets in the binary file
- `ROPgadget --binary path/to/binary --re regex` : filter gadgets in the binary file by a `regex`
- `ROPgadget --binary path/to/binary --norop|nojob|nosys` : list gadgets in the binary file, excluding specified type
- `ROPgadget --binary path/to/binary --badbytes byte_string` : exclude bad byte gadgets in the binary file
- `ROPgadget --binary path/to/binary --depth nbyte` : list gadgets up to the specified number of bytes in the binary file

## Categories
files

## Links
- More information: https://github.com/JonathanSalwan/ROPgadget#usage
- tldr page: https://tldr.inbrowser.app/pages/common/ropgadget
