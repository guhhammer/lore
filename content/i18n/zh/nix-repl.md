# nix repl

启动一个用于计算 Nix 表达式的交互式环境。

## Usage
- `nix repl` : 启动一个用于计算 Nix 表达式的交互式环境
- `:lf nixpkgs` : [交互式] 将所有包从某个 flake（例如 `nixpkgs`）加载到作用域中
- `:b expression` : [交互式] 从表达式构建一个软件包
- `:u expression` : [交互式] 启动一个包含表达式所提供软件包的 shell 环境
- `:s expression` : [交互式] 启动一个包含表达式所提供软件包依赖项的 shell 环境

## Links
- More information: https://nixos.org/manual/nix/stable/language/index.html
