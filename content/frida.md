# frida

A dynamic instrumentation toolkit for developers, reverse-engineers, and security researchers.

## Usage
- `frida process_name` : start the interactive shell (REPL) attached to a running process
- `frida -U process_name` : start the interactive shell attached to a process over USB
- `frida -p process_id` : attach to a running process by its PID
- `frida -l path/to/script.js process_name` : load a JavaScript script into a process
- `frida -c script_name process_name` : load a script from Frida Codeshare <https://codeshare.frida.re/>

## Categories
processes, development, network

## Links
- More information: https://frida.re/docs/frida-cli/
- tldr page: https://tldr.inbrowser.app/pages/common/frida
