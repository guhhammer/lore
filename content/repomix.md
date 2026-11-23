# repomix

Pack a GitHub repository into an AI-friendly file.

## Usage
- `repomix -o path/to/file --style xml|markdown|plain` : output custom format
- `repomix --stdout >path/to/file` : send output to `stdout`
- `repomix --stdout | less` : send output to `stdout`, then pipe into another program
- `repomix --compress` : output with compression
- `repomix --include "src/**/*.ts" --ignore "**/*.test.ts"` : process specific files
- `repomix --remote https://github.com/user/repo/tree/main` : pack a repository from a branch
- `repomix --remote https://github.com/user/repo/commit/836abcd7335137228ad77feb28655d85712680f1` : pack a repository at a specific commit
- `repomix --remote user/repo` : pack repository using shorthand

## Categories
files, packages, network

## Links
- More information: https://github.com/yamadashy/repomix
- tldr page: https://tldr.inbrowser.app/pages/common/repomix
