# test-json

Test whether a string is a valid JSON document. Note: This command can only be used through PowerShell.

## Usage
- `'string' | Test-Json` : test if a string from `stdin` is in JSON format
- `Test-Json -Json 'json_to_test'` : test if a string JSON format
- `'string' | Test-Json -SchemaFile path\to\schema_file.json` : test if a string from `stdin` matches a specific schema file

## Notes
A Windows command: this page is for reference, it does not run on Linux or macOS.

## Categories
text, development, documents, windows

## Links
- tldr page: https://tldr.inbrowser.app/pages/windows/test-json
