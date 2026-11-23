# msgattrib

Filter and manipulate message attributes in `.po` translation files.

## Usage
- `msgattrib --translated input.po >translated.po` : keep only translated messages
- `msgattrib --untranslated input.po >untranslated.po` : keep only untranslated messages
- `msgattrib --no-fuzzy input.po >clean.po` : remove fuzzy messages
- `msgattrib --only-fuzzy input.po >fuzzy.po` : keep only fuzzy messages
- `msgattrib --set-fuzzy input.po >fuzzy.po` : mark all messages as fuzzy
- `msgattrib --clear-fuzzy input.po >clean.po` : clear fuzzy marks
- `msgattrib -F input.po >sorted.po` : sort output by file location
- `msgattrib -h` : display help

## Links
- tldr page: https://tldr.inbrowser.app/pages/common/msgattrib
