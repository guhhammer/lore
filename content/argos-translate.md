# argos-translate

An open-source offline translation library and CLI tool written in Python.

## Usage
- `argospm install translate-es_en` : install translation pairs for Spanish to English translation
- `argos-translate --from-lang es --to-lang en un texto corto` : translate some text from Spanish (`es`) to English (`en`) (Note: Only two letter language codes are supported)
- `cat path/to/file.txt | argos-translate --from-lang en --to-lang hi` : translate a text file from English to Hindi
- `argospm list` : list all installed translation pairs
- `argospm search --from-lang en` : show translation pairs from English that are available to be installed
- `argospm update` : update installed language package pairs
- `argos-translate --from-lang ar --to-lang ru صورة تساوي أكثر من ألف كلمة` : translate from `ar` to `ru` (Note: This requires the translation pairs `translate-ar_en` and `translate-en_ru` to be installed)

## Categories
development, files, packages

## Links
- More information: https://argos-translate.readthedocs.io/en/latest/source/cli.html
- tldr page: https://tldr.inbrowser.app/pages/common/argos-translate
