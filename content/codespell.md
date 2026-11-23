# codespell

Spellchecker for source code.

## Usage
- `codespell` : check for typos in all text files in the current directory, recursively
- `codespell -w` : correct all typos found in-place
- `codespell -S "pattern"` : skip files with names that match the specified pattern (accepts a comma-separated list of patterns using wildcards)
- `codespell -D path/to/file.txt` : use a custom dictionary file when checking (`--dictionary` can be used multiple times)
- `codespell -I path/to/file.txt` : do not check words that are listed in the specified file
- `codespell -L ignored_word1,ignored_word2,...` : do not check the specified words
- `codespell --context|--before-context|--after-context 3` : print 3 lines of Context around, Before, or After each match
- `codespell -f` : check file names for typos, in addition to file contents

## Categories
files, text, development

## Links
- More information: https://manned.org/codespell
- tldr page: https://tldr.inbrowser.app/pages/common/codespell
