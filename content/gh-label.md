# gh label

Work with GitHub labels.

## Usage
- `gh label ls` : list labels for the repository in the current directory
- `gh label ls -w` : view labels for the repository in the current directory in the default web browser
- `gh label create name -d "description" -c color_hex` : create a label with a specific name, description, and color in hexadecimal format for the repository in the current directory
- `gh label delete name` : delete a label for the repository in the current directory, prompting for confirmation
- `gh label edit name -n new_name -d "description"` : update the name and description for a specific label for the repository in the current directory
- `gh label clone owner/repository` : clone labels from a specific repository into the repository in the current directory
- `gh label subcommand -h` : display help for a subcommand

## Categories
packages, files

## Links
- More information: https://cli.github.com/manual/gh_label
- tldr page: https://tldr.inbrowser.app/pages/common/gh-label
