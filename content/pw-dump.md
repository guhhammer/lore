# pw-dump

Dump PipeWire's current state as JSON, including the information on nodes, devices, modules, ports, and other objects.

## Usage
- `pw-dump` : print a JSON representation of the default PipeWire instance's current state
- `pw-dump object_id` : print a JSON representation of an object
- `pw-dump -m` : dump the current state monitoring changes, printing it again
- `pw-dump -r remote_name >path/to/dump_file.json` : dump the current state of a remote instance to a file
- `pw-dump -C never|always|auto` : set a color configuration
- `pw-dump -h` : display help

## See also
- pw-mon

## Categories
text, hardware

## Links
- More information: https://docs.pipewire.org/page_man_pw-dump_1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/pw-dump
