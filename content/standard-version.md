# standard-version

Automate versioning and changelog generation, with SemVer and Conventional Commits.

## Usage
- `standard-version` : update the changelog file and tag a release
- `standard-version --first-release` : tag a release without bumping the version
- `standard-version --prerelease alpha` : update the changelog and tag an alpha release
- `standard-version --release-as major|minor|patch` : update the changelog and tag a specific release type
- `standard-version --no-verify` : tag a release, preventing hooks from being verified during the commit step
- `standard-version --commit-all` : tag a release committing all staged changes, not just files affected by `standard-version`
- `standard-version --infile path/to/file.md` : update a specific changelog file and tag a release
- `standard-version --dry-run` : simulate a release, displaying what would be performed

## Categories
files, version-control

## Links
- More information: https://github.com/conventional-changelog/standard-version
- tldr page: https://tldr.inbrowser.app/pages/common/standard-version
