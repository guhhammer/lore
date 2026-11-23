# ni

Use the right package manager (npm, yarn, pnpm, bun, or deno) automatically. Detect the current project's lockfile to run the corresponding commands. Note: In PowerShell, `ni` is a built-in alias for `New-Item`. To use this tool, the alias must be removed.

## Usage
- `ni` : install all dependencies (equivalent to `npm install`, `yarn install`, etc.)
- `ni package` : install a specific package (-D for dev dependencies)
- `nr script` : run a `package.json` script (interactively select if no script is provided)
- `nlx package` : download and execute a command from a package (equivalent to `npx`, `pnpm dlx`, etc.)
- `nup` : upgrade dependencies
- `nun package` : uninstall a package
- `nci` : clean install dependencies (equivalent to `npm ci`, etc.)
- `na command` : use the current package manager agent directly for arbitrary commands

## Categories
packages, development, shell

## Links
- More information: https://github.com/antfu-collective/ni
- tldr page: https://tldr.inbrowser.app/pages/common/ni
