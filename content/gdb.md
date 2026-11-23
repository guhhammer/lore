# gdb

The GNU Debugger.

## Usage
- `gdb path/to/executable` : debug an executable
- `gdb -p process_id` : attach a process to `gdb`
- `gdb -c path/to/core path/to/executable` : debug with a core file
- `gdb -ex "commands" path/to/executable` : execute given GDB commands upon start
- `gdb --args path/to/executable argument1 argument2 ...` : start `gdb` and pass arguments to the executable
- `gdb -c path/to/core path/to/executable -iex 'set debuginfod enabled on' -iex 'set pagination off' -ex bt` : skip `debuginfod` and pagination prompts and then print the backtrace

## Categories
files, development

## Links
- More information: https://sourceware.org/gdb/current/onlinedocs/gdb#Invocation
- tldr page: https://tldr.inbrowser.app/pages/common/gdb
