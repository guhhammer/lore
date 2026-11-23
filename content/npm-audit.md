# npm audit

Scan project dependencies for known security vulnerabilities.

## Usage
- `npm audit` : scan the project's dependencies for known vulnerabilities
- `npm audit fix` : automatically fix vulnerabilities in the project's dependencies
- `npm audit fix -f` : force an automatic fix to dependencies with vulnerabilities
- `npm audit fix --package-lock-only` : update the lock file without modifying the `node_modules` directory
- `npm audit fix --dry-run` : simulate the fix process without making any changes
- `npm audit --json` : output audit results in JSON format
- `npm audit --audit-level info|low|moderate|high|critical` : configure the audit to only fail on vulnerabilities above a specified severity

## Categories
packages, development

## Links
- More information: https://docs.npmjs.com/cli/npm-audit/
- tldr page: https://tldr.inbrowser.app/pages/common/npm-audit
