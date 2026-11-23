# ropper

Find ROP gadgets in binary files.

## Usage
- `ropper --file path/to/binary` : list gadgets in the binary file
- `ropper --file path/to/binary --search regex` : filter gadgets in the binary file by a `regex`
- `ropper --file path/to/binary --type rop|job|sys|all` : list gadgets of specified type in the binary file
- `ropper --file path/to/binary --badbytes byte_string` : exclude bad byte gadgets in the binary file
- `ropper --file path/to/binary --inst-count count` : list gadgets up to the specified instruction count in the binary file

## Categories
files

## Links
- More information: https://scoding.de/ropper/
- tldr page: https://tldr.inbrowser.app/pages/common/ropper
