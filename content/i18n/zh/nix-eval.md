# nix eval

评估 Nix 表达式并将结果输出到标准输出。

## Usage
- `nix eval .#属性` : 在当前目录下评估 Nix flake 属性
- `nix eval --expr '你的表达式'` : 评估给定的 Nix 表达式
- `nix eval --file 文件路径` : 从指定文件中评估 Nix 表达式
- `nix eval nixpkgs#软件包名 --raw` : 打印 nixpkgs 中指定软件包的存储路径
- `nix eval github:所有者/仓库名#属性` : 直接从 GitHub 评估 flake 的属性
- `nix eval nixpkgs#软件包名 --apply 'lambda函数'` : 评估给定的 lambda 函数，并将指定的软件包作为参数传递

## Links
- More information: https://nix.dev/manual/nix/stable/command-ref/new-cli/nix3-eval.html
