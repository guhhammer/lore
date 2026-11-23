# monodis

The Mono Common Intermediate Language (CIL) disassembler.

## Usage
- `monodis path/to/assembly.exe` : disassemble an assembly to textual CIL
- `monodis --output=path/to/output.il path/to/assembly.exe` : save the output to a file
- `monodis --assembly path/to/assembly.dll` : show information about an assembly
- `monodis --assemblyref path/to/assembly.exe` : list the references of an assembly
- `monodis --method path/to/assembly.exe` : list all the methods in an assembly
- `monodis --manifest path/to/assembly.dll` : list resources embedded within an assembly
- `monodis --mresources path/to/assembly.dll` : extract all the embedded resources to the current directory

## Categories
files

## Links
- More information: https://www.mono-project.com/docs/tools+libraries/tools/monodis/
- tldr page: https://tldr.inbrowser.app/pages/common/monodis
