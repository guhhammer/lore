# pnpm audit

Scan project dependencies. Check for known security issues with the installed packages.

## Usage
- `pnpm audit` : identify vulnerabilities in the project
- `pnpm audit fix` : automatically fix vulnerabilities
- `pnpm audit --json >path/to/audit-report.json` : generate a security report in JSON format
- `pnpm audit -D` : audit only dev dependencies
- `pnpm audit -P` : audit only production dependencies
- `pnpm audit --no-optional` : exclude optional dependencies from the audit
- `pnpm audit --ignore-registry-errors` : ignore registry errors during the audit process
- `pnpm audit --audit-level severity` : filter advisories by severity (low, moderate, high, critical)

## Categories
packages, development, text

## Links
- More information: https://pnpm.io/cli/audit
- tldr page: https://tldr.inbrowser.app/pages/common/pnpm-audit
