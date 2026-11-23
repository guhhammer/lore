# jdeps

Java class dependency analyzer.

## Usage
- `jdeps path/to/file.class` : analyze the dependencies of a `.jar` or `.class` file
- `jdeps path/to/file.jar -summary` : print a summary of all dependencies of a specific `.jar` file
- `jdeps path/to/file.jar -verbose` : print all class-level dependencies of a `.jar` file
- `jdeps path/to/file.jar -dotoutput path/to/directory` : output the results of the analysis in a DOT file into a specific directory
- `jdeps --help` : display help

## Categories
files, packages

## Links
- More information: https://docs.oracle.com/en/java/javase/25/docs/specs/man/jdeps.html
- tldr page: https://tldr.inbrowser.app/pages/common/jdeps
