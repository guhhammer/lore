# bun create

Create a new project from a template.

## Usage
- `bun c template` : create a new project from an official template interactively
- `bun c template path/to/destination` : create a new project from an official template in a new directory
- `bun c https://github.com/username/repo path/to/destination` : create a new project from a GitHub repository template
- `bun c path/to/template path/to/destination` : create a new project from a local template
- `bun c template path/to/destination --force` : create a new project, overwriting the destination directory if it exists
- `bun c template path/to/destination --no-git` : create a new project without initializing a Git repository automatically
- `bun c template path/to/destination --no-install` : create a new project without installing dependencies automatically

## Categories
development, files, packages

## Links
- More information: https://bun.com/docs/runtime/templating/create
- tldr page: https://tldr.inbrowser.app/pages/common/bun-create
- Wikipedia: https://en.wikipedia.org/wiki/Bun_(software)
