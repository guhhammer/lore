# vp env

Manage Node.js and package manager runtime environments for Vite+.

## Usage
- `vp env setup` : create or update shims for Node.js and package managers
- `vp env current` : display the currently resolved environment and runtime versions
- `vp env use node_version` : activate a specific Node.js version in the current shell session
- `vp env pin node_version|package_manager@version` : pin Node.js or a package manager version for the current project
- `vp env unpin` : remove project environment pins
- `vp env doctor` : run diagnostics and verify environment configuration
- `vp env exec --node version command` : execute a command within a specific Node.js or package manager environment
- `vp env clean` : remove all unused managed runtime versions

## Categories
packages

## Links
- More information: https://viteplus.dev/guide/env
- tldr page: https://tldr.inbrowser.app/pages/common/vp-env
