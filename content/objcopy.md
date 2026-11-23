# objcopy

Copy the contents of an object file to another file.

## Usage
- `objcopy path/to/source_file path/to/target_file` : copy data to another file
- `objcopy --input-target=input_format --output-target output_format path/to/source_file path/to/target_file` : translate object files from one format to another
- `objcopy --strip-all path/to/source_file path/to/target_file` : strip all symbol information from the file
- `objcopy --strip-debug path/to/source_file path/to/target_file` : strip debugging information from the file
- `objcopy --only-section section path/to/source_file path/to/target_file` : copy a specific section from the source file to the destination file

## Categories
files

## Links
- More information: https://manned.org/objcopy
- tldr page: https://tldr.inbrowser.app/pages/linux/objcopy
