# bun audit

Check installed packages for known security vulnerabilities.

## Usage
- `bun audit` : audit all dependencies in a project with a `bun.lock` file
- `bun audit --audit-level low|moderate|high|critical` : show only vulnerabilities at or above a specific severity level
- `bun audit --prod` : audit only production dependencies
- `bun audit --ignore CVE-XXXX-YYYY` : ignore a specific CVE ID
- `bun audit --json` : output the raw JSON report

## Categories
packages

## Links
- More information: https://bun.com/docs/pm/cli/audit
- tldr page: https://tldr.inbrowser.app/pages/common/bun-audit
