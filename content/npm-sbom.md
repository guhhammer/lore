# npm sbom

Generate a Software Bill of Materials (SBOM) for your Node.js project.

## Usage
- `npm sbom` : output a list of all dependencies in your project
- `npm sbom --omit dev --omit optional` : exclude both `dev` and `optional` dependencies
- `npm sbom --package-lock-only` : generate an SBOM based only on the `package-lock.json`

## Categories
development, packages

## Links
- More information: https://docs.npmjs.com/cli/npm-sbom/
- tldr page: https://tldr.inbrowser.app/pages/common/npm-sbom
