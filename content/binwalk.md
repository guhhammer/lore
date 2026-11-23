# binwalk

Firmware Analysis Tool.

## Usage
- `binwalk path/to/binary` : scan a binary file
- `binwalk -e -C path/to/output_directory path/to/binary` : extract files from a binary, specifying the output directory
- `binwalk -e -M -d 2 path/to/binary` : recursively extract files from a binary limiting the recursion depth to 2
- `binwalk -D 'png image:png' path/to/binary` : extract files from a binary with the specified file signature
- `binwalk -E -J path/to/binary` : analyze the entropy of a binary, saving the plot with the same name as the binary and `.png` extension appended
- `binwalk -E -B -A path/to/binary` : combine entropy, signature, and opcodes analysis in a single command

## Categories
files, archives, hardware

## Links
- More information: https://manned.org/binwalk
- tldr page: https://tldr.inbrowser.app/pages/common/binwalk
