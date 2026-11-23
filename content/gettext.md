# gettext

Translate a string using stored translations in a compiled `.mo` file. Translations are stored in `/usr/share/locale/locale_name/LC_MESSAGES/` with `domain` being the filename without its extension.

## Usage
- `LANGUAGE=locale gettext -d domain "msgid"` : get the translation of a string as specified in the domain file (falls back to given `msgid` if no translation exists)
- `gettext -h` : display help
- `gettext -V` : display version

## See also
- msgfmt
- msgunfmt

## Categories
files, system, text

## Links
- More information: https://www.gnu.org/software/gettext/manual/gettext.html#gettext-Invocation
- tldr page: https://tldr.inbrowser.app/pages/common/gettext
