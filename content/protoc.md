# protoc

Parse Google Protobuf `.proto` files and generate output in the specified language.

## Usage
- `protoc --python_out=path/to/output_directory input_file.proto` : generate Python code from a `.proto` file
- `protoc --java_out=path/to/output_directory --proto_path=path/to/import_search_directory input_file.proto` : generate Java code from a `.proto` file that imports other `.proto` files
- `protoc --csharp_out=path/to/c#_output_directory --js_out=path/to/js_output_directory input_file.proto` : generate code for multiple languages
- `protoc <message.txt --encode=TypeName input_file.proto` : encode a text-format message into a protocol message from a `.proto` file
- `protoc <message.bin --decode=TypeName input_file.proto` : decode a protocol message into text-format from a `.proto` file
- `protoc <message.bin --decode_raw` : decode a protocol message into raw tag/value pairs

## Categories
files

## Links
- More information: https://manned.org/protoc
- tldr page: https://tldr.inbrowser.app/pages/common/protoc
