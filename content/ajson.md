# ajson

Execute JSONPath on JSON objects.

## Usage
- `ajson '$..json[?(@.path)]' path/to/file.json` : read JSON from a file and execute a specified JSONPath expression
- `cat path/to/file.json | ajson '$..json[?(@.path)]'` : read JSON from `stdin` and execute a specified JSONPath expression
- `ajson 'avg($..price)' 'https://example.com/api/'` : read JSON from a URL and evaluate a specified JSONPath expression
- `echo '3' | ajson '2 * pi * $'` : read some simple JSON and calculate a value

## Categories
text, files, network

## Links
- More information: https://github.com/spyzhov/ajson#console-application
- tldr page: https://tldr.inbrowser.app/pages/common/ajson
