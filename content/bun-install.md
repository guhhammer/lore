# bun install

Install JavaScript dependencies for a project from `package.json`.

## Usage
- `bun i` : install all dependencies listed in `package.json`
- `bun i package_name@version` : install a single package (this is an alias for `bun add`)
- `bun i -g package_name` : install a package globally
- `bun i -p` : install only production dependencies (skips `devDependencies`)
- `bun i --frozen-lockfile` : install dependencies exactly from the `bun.lockb` lockfile (frozen lockfile)
- `bun i -f` : force re-download all packages from the registry, ignoring the cache

## Categories
packages, text, development

## Links
- More information: https://bun.com/docs/pm/cli/install
- tldr page: https://tldr.inbrowser.app/pages/common/bun-install
- Wikipedia: https://en.wikipedia.org/wiki/Bun_(software)
