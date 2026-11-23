# sg

Ast-grep is a tool for code structural search, lint, and rewriting.

## Usage
- `sg scan --interactive` : scan for possible queries using interactive mode
- `sg run --pattern 'pattern' --rewrite 'replacement' --lang python` : rewrite code in the current directory using patterns
- `sg run --pattern 'useState<number>($A)' --rewrite 'useState($A)' --lang typescript` : visualize possible changes without applying them
- `sg run --pattern 'Some($A)' --rewrite 'None' --json | jq '.[].replacement' | jless` : output results as JSON, extract information using `jq`, and interactively view it using `jless`

## Categories
text

## Links
- More information: https://ast-grep.github.io/guide/introduction.html
- tldr page: https://tldr.inbrowser.app/pages/common/sg
