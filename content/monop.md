# monop

Find and display signatures of Types and methods inside .NET assemblies.

## Usage
- `monop System.String` : show the structure of a Type built-in of the .NET Framework
- `monop -r:path/to/assembly.exe` : list the types in an assembly
- `monop -r:path/to/assembly.dll Namespace.Path.To.Type` : show the structure of a Type in a specific assembly
- `monop -r:path/to/assembly.dll -d Namespace.Path.To.Type` : only show members defined in the specified Type
- `monop -r:path/to/assembly.dll -p Namespace.Path.To.Type` : show private members
- `monop -r:path/to/assembly.dll -f Namespace.Path.To.Type` : hide obsolete members
- `monop -r:path/to/assembly.dll --refs` : list the other assemblies that a specified assembly references

## Categories
files, desktop

## Links
- More information: https://manned.org/monop
- tldr page: https://tldr.inbrowser.app/pages/common/monop
