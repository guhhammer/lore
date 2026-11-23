# soupault

A static website generator based on HTML element tree rewriting. It can also be used as an HTML post-processor or metadata extractor.

## Usage
- `soupault --init` : initialize a minimal website project in the current working directory
- `soupault` : build a website
- `soupault --config path/to/config --site-dir path/to/input_directory --build-dir path/to/output_directory` : override default configuration file and directory locations
- `soupault --index-only --dump-index-json path/to/file.json` : extract metadata into a JSON file without generating pages
- `soupault --show-effective-config` : show the effective configuration (values from `soupault.toml` plus defaults)

## Categories
files, development, text

## Links
- More information: https://soupault.net/reference-manual/
- tldr page: https://tldr.inbrowser.app/pages/common/soupault
