# pw-container

Run a program in a new security context.

## Usage
- `pw-container` : create a new security context and print its socket address to `stdout`
- `pw-container command argument1 argument2 ...` : run a specific program within a new security context
- `pw-container -r remote_instance_name command` : run a program, connecting to a specific remote PipeWire instance
- `pw-container -P '{"key": "value"}' command` : run a program in a new context with specific properties using a JSON object
- `pw-container -h` : display help

## Categories
containers, network

## Links
- More information: https://docs.pipewire.org/page_man_pw-container_1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/pw-container
