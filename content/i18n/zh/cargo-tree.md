# cargo tree

显示依赖图的树形可视化。 注意：在树中，标有 `(*)` 的包的依赖已在图的其他位置显示过，因此不会重复显示。

## Usage
- `cargo tree` : 显示当前项目的依赖树
- `cargo tree --depth n` : 仅显示到指定深度的依赖 (例如，当 `n` 为 1 时，仅显示直接依赖)
- `cargo tree --prune package_spec` : 在树中不显示给定的包（及其依赖)
- `cargo tree --no-dedupe` : 显示重复依赖的所有出现
- `cargo tree -e normal|build|dev` : 仅显示 normal/build/dev 依赖

## Links
- More information: https://doc.rust-lang.org/cargo/commands/cargo-tree.html
