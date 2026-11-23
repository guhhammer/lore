# x86_64-linux-gnu-objcopy

Copy and translate object files.

## Usage
- `x86_64-linux-gnu-objcopy --output-target=bfdname` : write the output file using the object format bfdname
- `x86_64-linux-gnu-objcopy --strip-section-headers` : strip section header   This option is specific to ELF files
- `x86_64-linux-gnu-objcopy --strip-all` : do not copy relocation and symbol information from the source file
- `x86_64-linux-gnu-objcopy --strip-debug` : do not copy debugging symbols or sections from the source file

## Categories
files
