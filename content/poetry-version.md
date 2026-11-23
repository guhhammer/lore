# poetry version

Manage Poetry project version. Assumes the following project stages: `patch`, `minor`, `major`, `prepatch`, `preminor`, `premajor`, `prerelease`.

## Usage
- `poetry version -s` : output the current version
- `poetry version stage` : set project to a specific phase
- `poetry version --next-phase` : increment the project to the next prerelease phase
- `poetry version stage --dry-run` : simulate bumping the version to a given stage without writing to `pyproject.toml`

## See also
- asdf

## Categories
development, version-control

## Links
- More information: https://python-poetry.org/docs/cli/#version
- tldr page: https://tldr.inbrowser.app/pages/common/poetry-version
