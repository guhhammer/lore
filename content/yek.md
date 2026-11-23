# yek

Serialize a repository or directory into an LLM-friendly single file (fast Rust-based repomapper).

## Usage
- `yek` : serialize the current directory and write the output to a temp file (prints path)
- `yek path/to/directory1 path/to/directory2 ... --output-dir path/to/output_directory` : serialize specific directories and write output to a directory
- `yek "path/to/directory/**/*.rs" "path/to/directory/**/*.md"` : process multiple files or use `glob` patterns (quote `glob`s to avoid shell expansion)
- `yek path/to/directory --tokens 128k` : cap the token-based output size to 128k tokens
- `yek path/to/directory --max-size 100KB --output-name yek-output.txt` : cap the byte-based max output size and set an explicit output file name
- `yek path/to/directory --json` : stream JSON output
- `yek path/to/directory --tree-header` : include a directory tree header in the output

## Categories
files, packages

## Links
- More information: https://github.com/bodo-run/yek#usage
- tldr page: https://tldr.inbrowser.app/pages/common/yek
