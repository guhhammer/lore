# ld

Link object files together.

## Usage
- `ld path/to/file.o -o path/to/output_executable` : link a specific object file with no dependencies into an executable
- `ld path/to/file1.o path/to/file2.o ... -o path/to/output_executable` : link multiple object files together
- `ld path/to/file.o -m target_emulation -o path/to/output_executable` : link an object file to a specific target emulation
- `ld -o path/to/output_executable -I /lib/ld-linux-x86-64.so.2 /lib/crt1.o /lib/crti.o -lc path/to/file.o /lib/crtn.o` : dynamically link an x86_64 program to glibc (file paths change depending on the system)

## Categories
files

## Links
- More information: https://sourceware.org/binutils/docs/ld.html#Options
- tldr page: https://tldr.inbrowser.app/pages/common/ld
