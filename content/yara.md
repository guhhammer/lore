# yara

Pattern matching tool for identifying and classifying malware.

## Usage
- `yara path/to/rule.yar path/to/file` : scan a specific file with a rule file
- `yara path/to/rule.yar -r path/to/directory` : recursively scan a directory and subdirectories containing possible threats
- `yara path/to/rule1.yar path/to/rule2.yar ... process_id` : scan a running process by its PID using multiple rules
- `yara -m path/to/rule.yar path/to/file` : print metadata associated with the matching rules
- `yara -s path/to/rule.yar path/to/file` : print the strings that caused the rule to match
- `yara -p number_of_threads path/to/rule.yar path/to/directory` : use a specific number of threads for parallel scanning
- `yara -C path/to/rules.bin -r path/to/directory` : use compiled YARA rules file to scan a directory recursively

## See also
- yarac

## Categories
files, text

## Links
- More information: https://yara.readthedocs.io/en/stable/commandline.html
- tldr page: https://tldr.inbrowser.app/pages/common/yara
