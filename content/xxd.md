# xxd

Create a hexadecimal representation (hexdump) from a binary file, or vice-versa.

## Usage
- `xxd input_file` : generate a hexdump from a binary file and display the output
- `xxd input_file output_file` : generate a hexdump from a binary file and save it as a text file
- `xxd -a input_file` : display a more compact output, replacing consecutive zeros (if any) with a star
- `xxd -c 10 input_file` : display the output with 10 columns of one octet (byte) each
- `xxd -l 32 input_file` : display output only up to a length of 32 bytes
- `xxd -p input_file` : display the output in plain mode, without any gaps between the columns
- `xxd -r -p input_file output_file` : revert a plaintext hexdump back into binary, and save it as a binary file

## See also
- hexyl
- od
- hexdump

## Categories
files, desktop

## Links
- More information: https://manned.org/xxd
- tldr page: https://tldr.inbrowser.app/pages/common/xxd
