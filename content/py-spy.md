# py-spy

A sampling profiler for Python programs.

## Usage
- `py-spy top -p process_id` : show a live view of the functions that take the most execution time of a running process
- `py-spy top -- python path/to/file.py` : start a program and show a live view of the functions that take the most execution time
- `py-spy record -o path/to/profile.svg -p process_id` : produce an SVG flame graph of the function call execution time
- `py-spy dump -p process_id` : dump the call stack of a running process

## Categories
files

## Links
- More information: https://github.com/benfred/py-spy
- tldr page: https://tldr.inbrowser.app/pages/common/py-spy
