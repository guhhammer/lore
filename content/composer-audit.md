# composer audit

Analyze a PHP project's dependencies to detect known security vulnerabilities and list affected packages.

## Usage
- `composer audit` : check for security vulnerabilities in your current project
- `composer audit --no-dev` : omit dev dependencies in the audit
- `composer audit --format table|plain|json|summary` : filter vulnerabilities by output format
- `composer audit --format json >audit_report.json` : output audit results to a file in JSON format
- `composer audit vendor/package` : verify whether a specific package in your project is affected by security issues

## Categories
packages, development, text

## Links
- More information: https://getcomposer.org/doc/03-cli.md#audit
- tldr page: https://tldr.inbrowser.app/pages/common/composer-audit
