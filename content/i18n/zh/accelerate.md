# accelerate

一个使得可以在任何分布式配置中运行相同的 PyTorch 代码的库。

## Usage
- `accelerate env` : 打印环境信息
- `accelerate config` : 交互式地创建配置文件
- `accelerate estimate-memory 名字/模型` : 打印使用不同数据类型运行 Hugging Face 模型的估计 GPU 内存成本
- `accelerate test --config_file 路径/到/配置文件.yaml` : 测试一个 Accelerate 配置文件
- `accelerate launch 路径/到/脚本.py --cpu` : 使用 Accelerate 在 CPU 上运行一个模型
- `accelerate launch 路径/到/脚本.py --multi_gpu --num_machines 2` : 使用 Accelerate 在多 GPU 上运行一个模型，使用 2 台机器

## Links
- More information: https://huggingface.co/docs/accelerate/index
