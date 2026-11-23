# code2prompt

Generate AI-ready prompts from a codebase (extract, filter, and format code for LLMs).

## Usage
- `code2prompt path/to/project` : generate a prompt for the current project and copy it to the clipboard (default behavior)
- `code2prompt path/to/project -i "**/*.rs" -e "tests/**"` : include only specific files and exclude a directory
- `code2prompt path/to/project -O my_prompt.txt` : write the prompt to a file instead of the clipboard
- `code2prompt path/to/project -F json` : produce structured JSON output
- `code2prompt path/to/project -t my_template.hbs` : use a custom Handlebars template when generating the prompt

## Categories
files, development, archives

## Links
- More information: https://code2prompt.dev/docs/how_to/filter_files/
- tldr page: https://tldr.inbrowser.app/pages/common/code2prompt
