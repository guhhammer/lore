# jlink

Builds a small custom Java runtime that contains only the modules an application needs.

## Usage
- `jlink --add-modules java.base --output rt` : create a runtime with the base module only
- `jlink --add-modules java.base,java.sql --output rt` : include more modules
- `jlink --list-plugins` : show the optimisation plugins available
