# hurl

Run and test HTTP requests defined in a simple plain text format. Powered by `curl`.

## Usage
- `hurl path/to/file.hurl` : run HTTP requests from a file
- `hurl --variable variable_name=value path/to/file.hurl` : run HTTP requests and set variable to use
- `hurl --secret secret_name=value path/to/file.hurl` : run HTTP requests with secret to be redacted on logs and reports
- `hurl --variables-file path/to/variables_file --secrets-file path/to/secrets_file path/to/file.hurl` : run HTTP requests and inject variables and secret from file
- `hurl --from-entry 2 --to-entry 5 path/to/file.hurl` : run specific HTTP requests from file, from entry 2 to 5
- `hurl --test --report-html path/to/output_directory path/to/file.hurl` : test HTTP requests from file and generate report in HTML

## Categories
files, network, development

## Links
- More information: https://hurl.dev/docs/manual.html
- tldr page: https://tldr.inbrowser.app/pages/common/hurl
