# nix edit

在 `$EDITOR` 中打开 Nix 包的 Nix 表达式文件。

## Usage
- `nix edit nixpkgs#软件包` : 在 `$EDITOR` 中打开 nixpkgs 中指定包的 Nix 表达式源码
- `EDITOR=cat nix edit nixpkgs#软件包` : 将包的源码直接打印输出至标准输出 (`stdout`)

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-edit.html
