# frida-trace

Dynamically trace function calls.

## Usage
- `frida-trace -i "wildcard" process_name` : trace all functions matching a pattern in a process
- `frida-trace -i "function_name" process_name` : trace a specific function in a process
- `frida-trace -I "module_name" process_name` : trace all functions in a specific module
- `frida-trace -U -i "function_name" process_name` : trace a function on a USB-connected device
- `frida-trace -f path/to/executable -i "function_name"` : spawn a process and trace a function

## Categories
processes

## Links
- More information: https://frida.re/docs/frida-trace/
- tldr page: https://tldr.inbrowser.app/pages/common/frida-trace
