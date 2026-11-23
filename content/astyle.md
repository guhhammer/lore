# astyle

Source code indenter, formatter, and beautifier for the C, C++, C#, and Java programming languages. Upon running, a copy of the original file is created with an `.orig` appended to the original file name.

## Usage
- `astyle source_file` : apply the default style of 4 spaces per indent and no formatting changes
- `astyle -A2 path/to/file` : apply the Java style with attached braces
- `astyle -A1 path/to/file` : apply the allman style with broken braces
- `astyle -s2..20 path/to/file` : apply a custom indent using spaces. Choose between 2 and 20 spaces
- `astyle -t2..20 path/to/file` : apply a custom indent using tabs. Choose between 2 and 20 tabs

## Categories
files, development

## Links
- More information: https://manned.org/astyle
- tldr page: https://tldr.inbrowser.app/pages/common/astyle
