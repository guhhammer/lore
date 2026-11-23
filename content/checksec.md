# checksec

Check security properties of executables. Note: For older versions, use flags (e.g., `--file` instead of `file`).

## Usage
- `checksec file path/to/binary` : list security properties of an executable binary file
- `checksec dir path/to/directory` : list security properties recursively of all executable files in a directory
- `checksec proc process_id` : list security properties of a process
- `checksec kernel` : list security properties of the running kernel

## Categories
files

## Links
- More information: https://github.com/slimm609/checksec
- tldr page: https://tldr.inbrowser.app/pages/common/checksec
