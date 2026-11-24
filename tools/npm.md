# npm

npm installs JavaScript packages and runs a project's scripts. It comes with Node.js.

## Tasks
### Start a project
- `npm init -y` : make a package.json with defaults
- `npm install` : install everything package.json lists
- `npm ci` : clean install exactly from package-lock.json (for CI)
### Add and remove packages
- `npm install express` : add a dependency
- `npm install -D typescript` : add a development dependency
- `npm install express@4` : a chosen version
- `npm install -g NAME` : install a tool globally
- `npm uninstall express` : remove a package
- `npm update` : update packages within their allowed range
### Run things
- `npm run` : list the project's scripts
- `npm run build` : run the build script
- `npm start` : run the start script
- `npm test` : run the test script
- `npx cowsay hello` : run a package without installing it
- `npm run dev -- --port 3000` : pass extra arguments to a script
### Look
- `npm ls` : the dependency tree
- `npm ls -g --depth=0` : global packages
- `npm outdated` : packages with newer versions
- `npm view express version` : the newest version on the registry
- `npm audit` : check for known vulnerabilities
- `npm audit fix` : apply safe fixes
### Publish and settings
- `npm login` : log in to the registry
- `npm publish --access public` : publish a package
- `npm pack` : build the tarball that would be published
- `npm config get registry` : which registry is in use
- `npm cache verify` : check and clean the cache

## Notes
- Commit `package-lock.json`; never commit `node_modules`.
- `-g` installs may need root with a system Node: use nvm or set a prefix in your home instead of sudo.
- `npm install` can change the lock file; `npm ci` never does.
- `--` separates npm's flags from the script's own flags.

## See also
- pnpm
- nvm
- node
- yarn

## Links
- Docs: https://docs.npmjs.com/cli/
- Registry: https://www.npmjs.com

## Categories
packages development
