# pw-dot

Create `.dot` files of the PipeWire graph.

## Usage
- `pw-dot` : generate a graph to `pw.dot` file
- `pw-dot -j path/to/file.json` : read objects from `pw-dump` JSON file
- `pw-dot -o path/to/file.dot -a` : specify an output file, showing all object types
- `pw-dot -o - -d` : print `.dot` graph to `stdout`, showing all object properties
- `pw-dot -r remote_name -s` : generate a graph from a remote instance, showing only linked objects
- `pw-dot -L` : lay the graph from left to right, instead of dot's default top to bottom
- `pw-dot -9` : lay the graph using 90-degree angles in edges
- `pw-dot -h` : display help

## See also
- dot

## Categories
files

## Links
- More information: https://docs.pipewire.org/page_man_pw-dot_1.html
- tldr page: https://tldr.inbrowser.app/pages/linux/pw-dot
