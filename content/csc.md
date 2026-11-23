# csc

The Microsoft C# Compiler.

## Usage
- `csc path/to/input_file1.cs path/to/input_file2.cs ...` : compile one or more C# files to a CIL executable
- `csc /out:path/to/file path/to/input_file.cs` : specify the output filename
- `csc /target:library path/to/input_file.cs` : compile into a `.dll` library instead of an executable
- `csc /reference:path/to/library.dll path/to/input_file.cs` : reference another assembly
- `csc /resource:path/to/resource_file path/to/input_file.cs` : embed a resource
- `csc /doc:path/to/output.xml path/to/input_file.cs` : automatically generate XML documentation
- `csc /win32icon:path/to/icon.ico path/to/input_file.cs` : specify an icon
- `csc /keyfile:path/to/keyfile path/to/input_file.cs` : strongly-name the resulting assembly with a keyfile

## Categories
files, development

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/csc
