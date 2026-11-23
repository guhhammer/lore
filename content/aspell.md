# aspell

Interactive spell checker.

## Usage
- `aspell check path/to/file` : spell check a single file
- `cat path/to/file | aspell list` : list misspelled words from `stdin`
- `aspell dicts` : show available dictionary languages
- `aspell --lang cs` : run `aspell` with a different language (takes two-letter ISO 639 language code)
- `cat path/to/file | aspell --personal personal-word-list.pws list` : list misspelled words from `stdin` and ignore words from personal word list

## Categories
files

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/aspell
