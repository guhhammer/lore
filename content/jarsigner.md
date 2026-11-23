# jarsigner

Sign and verify Java archive (`.jar`) files.

## Usage
- `jarsigner path/to/file.jar keystore_alias` : sign a `.jar` file
- `jarsigner -sigalg algorithm path/to/file.jar keystore_alias` : sign a `.jar` file with a specific algorithm
- `jarsigner -verify path/to/file.jar` : verify the signature of a `.jar` file

## Categories
files, archives

## Links
- More information: https://docs.oracle.com/en/java/javase/25/docs/specs/man/jarsigner.html
- tldr page: https://tldr.inbrowser.app/pages/common/jarsigner
