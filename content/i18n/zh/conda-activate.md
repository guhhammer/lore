# conda activate

激活一个 conda 环境。

## Usage
- `conda activate myenv` : 激活现存名为 `myenv` 的环境
- `conda activate 路径/到/我的环境` : 激活位于特定路径的环境
- `conda activate --stack myenv` : 将 `myenv` 环境堆积在先前的环境上，让两个环境的程序库/命令/变量都能够被访问
- `conda activate --no-stack myenv` : 启动干净的 `myenv` 环境，不执行堆积（先前环境中的程序库/命令/变量将不可访问）
- `conda activate -h` : 显示帮助
