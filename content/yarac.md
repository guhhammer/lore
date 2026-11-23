# yarac

Compile YARA rule source files into a binary format for faster loading.

## Usage
- `yarac path/to/rule.yar path/to/rule.bin` : compile a specific rule file
- `yarac path/to/rule1.yar path/to/rule2.yar ... path/to/rules.bin` : compile multiple rule files into a single binary
- `yarac -d identifier=value path/to/rule.yar path/to/rule.bin` : define an external variable during compilation
- `yarac -w path/to/rule.yar path/to/rule.bin` : disable warnings during compilation
- `yarac --fail-on-warnings path/to/rule.yar path/to/rule.bin` : fail compilation on any warnings (do not use along with `--no-warnings`)

## See also
- yara

## Categories
files, development

## Links
- More information: https://manned.org/yarac
- tldr page: https://tldr.inbrowser.app/pages/common/yarac
