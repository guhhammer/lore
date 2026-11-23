# phpmd

PHP mess detector: check for common potential problems.

## Usage
- `phpmd` : display a list of available rulesets and formats
- `phpmd path/to/file_or_directory xml|text|html ruleset1,ruleset2,...` : scan a file or directory for problems using comma-separated rulesets
- `phpmd path/to/file_or_directory xml|text|html ruleset1,ruleset2,... --minimumpriority priority` : specify the minimum priority threshold for rules
- `phpmd path/to/file_or_directory xml|text|html ruleset1,ruleset2,... --suffixes extensions` : include only the specified extensions in analysis
- `phpmd path/to/file_or_directory1,path/to/file_or_directory2,... xml|text|html ruleset1,ruleset2,... --exclude directory_patterns` : exclude the specified comma-separated directories
- `phpmd path/to/file_or_directory xml|text|html ruleset1,ruleset2,... --reportfile path/to/report_file` : output the results to a file instead of `stdout`
- `phpmd path/to/file_or_directory xml|text|html ruleset1,ruleset2,... --strict` : ignore the use of warning-suppressive PHPDoc comments

## Categories
text, files

## Links
- More information: https://github.com/phpmd/phpmd#command-line-options
- tldr page: https://tldr.inbrowser.app/pages/common/phpmd
