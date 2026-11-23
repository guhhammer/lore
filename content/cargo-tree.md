# cargo tree

Display a tree visualization of a dependency graph. Note: In the tree, dependencies of packages marked with `(*)` have already been shown elsewhere in the graph, and so are not repeated.

## Usage
- `cargo tree` : show a dependency tree of the current project
- `cargo tree --depth n` : only show dependencies up to the specified depth (e.g. when `n` is 1, display only direct dependencies)
- `cargo tree --prune package_spec` : do not display the given package (and its dependencies) in the tree
- `cargo tree --no-dedupe` : show all occurrences of repeated dependencies
- `cargo tree -e normal|build|dev` : only show normal/build/development dependencies

## Categories
packages, desktop, development

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-tree.html
- tldr page: https://tldr.inbrowser.app/pages/common/cargo-tree
