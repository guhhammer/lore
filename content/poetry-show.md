# poetry show

Show details of packages in your Poetry project.

## Usage
- `poetry show` : display all packages
- `poetry show package_name` : show details of a specific package
- `poetry show -t` : show details as a dependency tree
- `poetry show -T` : only show top-level packages (those explicitly defined in `pyproject.toml`)
- `poetry show -o` : show outdated packages
- `poetry show -l` : show the latest versions for all packages
- `poetry show --without group1,group2,...` : exclude a specific dependency group/s
- `poetry show --only group1,group2,...` : only show a specific dependency group/s

## See also
- asdf

## Categories
packages, development

## Links
- More information: https://python-poetry.org/docs/cli/#show
- tldr page: https://tldr.inbrowser.app/pages/common/poetry-show
