# pnpm

pnpm is a fast npm replacement that keeps one copy of each package on disk and links it into projects. Its commands mirror npm's.

## Tasks
### Install
- `pnpm install` : install what package.json and the lock file list
- `pnpm install --frozen-lockfile` : fail if the lock file would change (for CI)
- `pnpm add express` : add a dependency
- `pnpm add -D typescript` : add a development dependency
- `pnpm add -g NAME` : install a tool globally
- `pnpm remove express` : remove a package
### Update
- `pnpm update` : update within the allowed ranges
- `pnpm update --latest` : go to the newest versions, ignoring ranges
- `pnpm up -i` : choose interactively what to update
- `pnpm outdated` : show packages with newer versions
### Run
- `pnpm run build` : run a script
- `pnpm dev` : run the dev script (short form for any script)
- `pnpm exec eslint .` : run a binary from the project
- `pnpm dlx create-vite` : run a package once without installing it
- `pnpm test` : run the tests
### Look
- `pnpm list` : the dependency tree
- `pnpm why NAME` : why a package is installed
- `pnpm audit` : check for known vulnerabilities
- `pnpm store path` : where the shared store lives
- `pnpm store prune` : remove unused packages from the store
### Workspaces
- `pnpm -r build` : run a script in every workspace package
- `pnpm --filter NAME build` : run it in one package
- `pnpm add NAME --filter APP` : add a dependency to one package
- `pnpm import` : make a pnpm-lock.yaml from an npm or yarn lock

## Notes
- Commit `pnpm-lock.yaml`; do not mix it with package-lock.json.
- `node_modules` holds links, not copies: tools that ignore symlinks may need `node-linker=hoisted` in `.npmrc`.
- Enable it with `corepack enable` (comes with Node) or follow the official install page.
- A package can only use what it lists as a dependency, which catches missing entries early.

## See also
- npm
- nvm
- node
- yarn

## Links
- Docs: https://pnpm.io/cli/install
- Install: https://pnpm.io/installation

## Categories
packages development
