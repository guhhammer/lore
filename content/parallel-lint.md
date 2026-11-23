# parallel-lint

Check the syntax of PHP files in parallel.

## Usage
- `parallel-lint path/to/directory` : lint a specific directory
- `parallel-lint -j processes path/to/directory` : lint a directory using the specified number of parallel processes
- `parallel-lint --exclude path/to/excluded_directory path/to/directory` : lint a directory, excluding the specified directory
- `parallel-lint -e php,html,phpt path/to/directory` : lint a directory of files using a comma-separated list of extension(s)
- `parallel-lint --json path/to/directory` : lint a directory and output the results as JSON
- `parallel-lint --blame path/to/directory` : lint a directory and show Git Blame results for rows containing errors

## Categories
files

## Links
- More information: https://github.com/JakubOnderka/PHP-Parallel-Lint
- tldr page: https://tldr.inbrowser.app/pages/common/parallel-lint
